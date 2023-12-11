use components::drawable_components::{Position, Sprite};
use components::movement_components::Velocity;

use robotics_lib::world::tile::TileType;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::Sdl;
use systems::movement_systems::MoveSystem;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use specs::{Builder, Dispatcher, DispatcherBuilder, World, WorldExt};

use robotics_lib::interface::Direction;
use robotics_lib::runner::Runner;

use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

use crate::texture_manager::TextureType;

mod components;
mod renderer;
mod systems;
mod texture_manager;

const HEIGHT: u32 = 600;
const WIDTH: u32 = 800;

struct MainState<'a> {
    sdl_context: Sdl,
    //window: Window,
    canvas: Canvas<Window>,
    game_world: World,
    robot_world: World,
    dispatcher: Dispatcher<'a, 'a>,
    textures: HashMap<TextureType, Box<Vec<Texture<'a>>>>,
    texture_creator: TextureCreator<WindowContext>,
}

impl<'a> MainState<'a> {
    pub fn init(&mut self, _run: &Runner) -> Result<(), String> {
        self.sdl_context = sdl2::init()?;

        let window = self
            .sdl_context
            .video()?
            .window("ROBOTICS", WIDTH, HEIGHT)
            .position_centered()
            .build()
            .expect("could not initialize window");

        self.canvas = window
            .into_canvas()
            .build()
            .expect("could not create canvas");

        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

        self.texture_creator = self.canvas.texture_creator();

        let robot_texture = self.texture_creator.load_texture(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("assets")
                .join("bardo.png"),
        )?;
        let grass_texture = self.texture_creator.load_texture(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("assets")
                .join("Texture")
                .join("TX Tileset Grass.png"),
        )?;
        self.textures = HashMap::new();
        self.textures
            .insert(TextureType::Robot, Box::new(vec![robot_texture]));
        self.textures.insert(
            TextureType::Tile(TileType::Grass),
            Box::new(vec![grass_texture]),
        );

        self.game_world = World::new();
        self.game_world.register::<Position>();
        self.game_world.register::<Sprite>();

        self.robot_world = World::new();
        self.robot_world.register::<Velocity>();
        self.robot_world.register::<Position>();
        self.robot_world.register::<Sprite>();

        //robot
        self.robot_world
            .create_entity()
            .with(Position(Point::new(0, 0)))
            .with(Velocity {
                speed: 1,
                direction: Direction::Up,
            })
            .with(Sprite {
                region: Rect::new(0, 0, 26, 39),
                sprite_type: components::drawable_components::SpriteType::Robot,
            })
            .build();

        //grass tile
        self.game_world
            .create_entity()
            .with(Position(Point::new(0, 0)))
            .with(Sprite {
                region: Rect::new(0, 0, 26, 39),
                sprite_type: components::drawable_components::SpriteType::Tile,
            })
            .build();

        //chiama i system relativi al robot
        self.dispatcher = DispatcherBuilder::new()
            .with(MoveSystem, "Movement", &[])
            .build();

        self.dispatcher.setup(&mut self.robot_world);
        Ok(())
    }

    pub fn start(&mut self) -> Result<(), String> {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        'running: loop {
            //Event handling
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown {
                        keycode: Some(Keycode::Down),
                        ..
                    } => {
                        println!("down");
                    }
                    _ => {}
                }
            }

            //UPDATE
            self.dispatcher.dispatch(&self.robot_world);
            self.game_world.maintain();
            self.robot_world.maintain();

            //render_world(&mut canvas, &grass_texture, &tile_list)?;
            //render_robot(&mut canvas, bg, &reaper_texture, &player_list)?;

            //chiamare più volte il rendere per ogni tipo di cosa da renderizzare
            //
            self.canvas.clear();
            let _ = renderer::render(
                &mut self.canvas,
                &self.textures,
                self.game_world.system_data(),
            );
            //let _ = renderer::render(&mut self.canvas, textu, self.robot_world.system_data());
            //let rendtest: DispatcherBuilder = DispatcherBuilder::new();
            self.canvas.present();

            //Time mgmt
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
        Ok(())
    }
}
