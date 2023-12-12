use components::drawable_components::{Position, Sprite};
use components::movement_components::Velocity;

use robotics_lib::world::tile::{Content, Tile, TileType};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::Sdl;
use systems::movement_systems::{ChangeDirectionSystem, MoveSystem};

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use specs::{Builder, Dispatcher, DispatcherBuilder, World, WorldExt};

use robotics_lib::interface::Direction;
use texture_manager::{SpriteTable, Textures};

use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

use crate::texture_manager::TextureType;

mod components;
mod renderer;
mod systems;
mod texture_manager;

const HEIGHT: u32 = 720;
const WIDTH: u32 = 1280;

pub struct MainState<'window> {
    sdl_context: Sdl,
    //window: Window,
    canvas: Canvas<Window>,
    game_world: World,
    robot_world: World,
    dispatcher: Dispatcher<'window, 'window>,
    //textures: HashMap<TextureType, Box<Vec<Texture<'window>>>>,
    texture_creator: TextureCreator<WindowContext>,
    sprite_table: SpriteTable,
}

impl<'window> MainState<'window> {
    pub fn new() -> Result<MainState<'window>, String> {
        let sdl_context = sdl2::init()?;

        let window = sdl_context
            .video()?
            .window("ROBOTICS", WIDTH, HEIGHT)
            .position_centered()
            .build()
            .expect("could not initialize window");

        let canvas = window
            .into_canvas()
            .build()
            .expect("could not create canvas");

        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

        let texture_creator = canvas.texture_creator();

        let mut game_world = World::new();
        game_world.register::<Position>();
        game_world.register::<Sprite>();

        let mut robot_world = World::new();
        robot_world.register::<Velocity>();
        robot_world.register::<Position>();
        robot_world.register::<Sprite>();

        //robot
        robot_world
            .create_entity()
            .with(Position(Point::new(0, 0)))
            .with(Velocity {
                speed: 1,
                direction: None,
            })
            .with(Sprite {
                region: Rect::new(0, 0, 26, 39),
                texture_type: TextureType::Robot,
            })
            .build();

        //grass tile
        game_world
            .create_entity()
            .with(Position(Point::new(0, 0)))
            .with(Sprite {
                region: Rect::new(32, 32 * 3, 32, 32),
                texture_type: TextureType::Tile(TileType::Grass),
            })
            .build();

        //chiama i system relativi al robot
        let mut dispatcher = DispatcherBuilder::new()
            .with(ChangeDirectionSystem, "ChangeDir", &[])
            .with(MoveSystem, "Movement", &["ChangeDir"])
            .build();

        dispatcher.setup(&mut robot_world);

        let mut sprite_table = SpriteTable::new();

        sprite_table
            .0
            .insert(TextureType::Robot, Rect::new(0, 0, 26, 39));
        sprite_table.0.insert(
            TextureType::Tile(TileType::Grass),
            Rect::new(32, 32 * 3, 32, 32),
        );
        sprite_table.0.insert(
            TextureType::Tile(TileType::Sand),
            Rect::new(0, 32 * 6, 32, 32),
        );
        sprite_table.0.insert(
            TextureType::Content(Content::Rock(0)),
            Rect::new(0, 32 * 15, 32, 32),
        );

        Ok(MainState {
            sdl_context,
            canvas,
            game_world,
            robot_world,
            dispatcher,
            texture_creator,
            sprite_table,
        })
    }

    pub fn start(&mut self) -> Result<(), String> {
        let mut textures = Textures::new();

        let grass_texture = self.texture_creator.load_texture(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("assets")
                .join("tiles")
                .join("grass.png"),
        )?;

        let robot_texture = self.texture_creator.load_texture(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("assets")
                .join("bardo.png"),
        )?;

        let _ = textures
            .add_texture(TextureType::Tile(TileType::Grass), &grass_texture)
            .clone();
        let _ = textures
            .add_texture(TextureType::Robot, &robot_texture)
            .clone();

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

            //chiamare più volte il rendere per ogni tipo di cosa da renderizzare
            //
            self.canvas.clear();

            let _ = renderer::render(&mut self.canvas, &textures, self.game_world.system_data());
            let _ = renderer::render(&mut self.canvas, &textures, self.robot_world.system_data());

            self.canvas.present();

            //Time mgmt
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
        Ok(())
    }

    pub fn update_world(&mut self, world: Vec<Vec<Option<Tile>>>) {
        self.game_world.delete_all();
        let mut y = 0;
        let mut x;

        for rows in world.iter() {
            x = 0;
            for cols in rows {
                match cols {
                    Some(t) => {
                        self.game_world
                            .create_entity()
                            .with(Position(Point::new(x * 32, y * 32)))
                            .with(Sprite {
                                region: *self
                                    .sprite_table
                                    .0
                                    .get(&TextureType::Tile(t.tile_type))
                                    .unwrap(),
                                texture_type: TextureType::Tile(t.tile_type),
                            })
                            .build();
                        match t.content {
                            Content::None => {}
                            Content::Rock(_) => {
                                self.game_world
                                    .create_entity()
                                    .with(Position(Point::new(x * 32, y * 32)))
                                    .with(Sprite {
                                        region: *self
                                            .sprite_table
                                            .0
                                            .get(&TextureType::Content(Content::Rock(0)))
                                            .unwrap(),
                                        texture_type: TextureType::Content(Content::Rock(0)),
                                    })
                                    .build();
                            }
                            _ => {}
                        }
                    }
                    None => print!(" ######### "),
                }
                x += 1;
            }
            y += 1;
            println!();
        }
    }

    pub fn update_robot(&mut self, dir: robotics_lib::interface::Direction) {
        self.robot_world.insert(Some(dir));
    }

    pub fn tick(&mut self) -> Result<(), String> {
        let mut textures = Textures::new();

        let grass_texture = self.texture_creator.load_texture(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("assets")
                .join("tiles")
                .join("grass.png"),
        )?;

        let robot_texture = self.texture_creator.load_texture(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("assets")
                .join("bardo.png"),
        )?;
        let sand_texture = self.texture_creator.load_texture(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("assets")
                .join("tiles")
                .join("grass.png"),
        )?;
        let rock_texture = self.texture_creator.load_texture(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("assets")
                .join("tiles")
                .join("props.png"),
        )?;

        let _ = textures
            .add_texture(TextureType::Tile(TileType::Grass), &grass_texture)
            .clone();
        let _ = textures
            .add_texture(TextureType::Robot, &robot_texture)
            .clone();
        let _ = textures
            .add_texture(TextureType::Tile(TileType::Sand), &sand_texture)
            .clone();
        let _ = textures
            .add_texture(TextureType::Content(Content::Rock(0)), &rock_texture)
            .clone();

        for _i in 0..32 {
            let mut event_pump = self.sdl_context.event_pump().unwrap();
            //Event handling
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {}
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

            //chiamare più volte il rendere per ogni tipo di cosa da renderizzare
            //
            self.canvas.clear();

            let _ = renderer::render(&mut self.canvas, &textures, self.game_world.system_data());
            let _ = renderer::render(&mut self.canvas, &textures, self.robot_world.system_data());

            self.canvas.present();

            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }
}
