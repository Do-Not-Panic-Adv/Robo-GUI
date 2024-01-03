use components::drawable_components::{Position, Sprite};
use components::movement_components::Velocity;

use robotics_lib::interface::Direction;
use robotics_lib::world::tile::{Content, Tile, TileType};
use sdl2::mouse::MouseState;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::Sdl;
use systems::movement_systems::{ChangeDirectionSystem, MoveSystem};

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use specs::{Builder, Dispatcher, DispatcherBuilder, World, WorldExt};

use texture_manager::{SpriteTable, Textures};

use std::path::Path;
use std::time::Duration;

use crate::texture_manager::TextureType;

mod components;
mod renderer;
mod systems;
mod texture_manager;

const HEIGHT: u32 = 720;
const WIDTH: u32 = 1280;

const TILE_SIZE: i32 = 32;
static ZOOM_LEVEL: i32 = 1;

pub struct MainState<'window> {
    sdl_context: Sdl,
    //window: Window,
    canvas: Canvas<Window>,
    game_world: World,
    robot_world: World,
    content_world: World,
    dispatcher: Dispatcher<'window, 'window>,
    //textures: HashMap<TextureType, Box<Vec<Texture<'window>>>>,
    texture_creator: TextureCreator<WindowContext>,
    sprite_table: SpriteTable,
    screen_offset: (i32, i32), //maybe move this outside of the mainstate
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

        let mut content_world = World::new();
        content_world.register::<Position>();
        content_world.register::<Sprite>();

        robot_world.insert(Some(Direction::Right));
        //game_world.insert(Some(Direction::Right));
        //content_world.insert(Some(Direction::Right));

        //robot entity
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

        //grass entity
        //Creata automaticamente quando si aggiorna il mondo
        //game_world .create_entity() .with(Position(Point::new(0, 0))) .with(Sprite { region: Rect::new(TILE_SIZE, TILE_SIZE * 3, TILE_SIZE as u32, TILE_SIZE as u32), texture_type: TextureType::Tile(TileType::Grass), }) .build();
        //
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
            Rect::new(
                TILE_SIZE * 1,
                TILE_SIZE * 1,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );
        sprite_table.0.insert(
            TextureType::Tile(TileType::Sand),
            Rect::new(
                TILE_SIZE * 1,
                TILE_SIZE * 2,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );
        sprite_table.0.insert(
            TextureType::Content(Content::Rock(0)),
            Rect::new(0, TILE_SIZE * 15, TILE_SIZE as u32, TILE_SIZE as u32),
        );
        sprite_table.0.insert(
            TextureType::Tile(TileType::Street),
            Rect::new(
                TILE_SIZE * 2,
                TILE_SIZE * 2,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );

        let screen_offset = (0, 0);
        Ok(MainState {
            sdl_context,
            canvas,
            game_world,
            robot_world,
            content_world,
            dispatcher,
            texture_creator,
            sprite_table,
            screen_offset,
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
        let road_texture = self.texture_creator.load_texture(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("assets")
                .join("tiles")
                .join("street.png"),
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
        let _ = textures
            .add_texture(TextureType::Tile(TileType::Street), &road_texture)
            .clone();

        'running: loop {
            let mut event_pump = self.sdl_context.event_pump().unwrap();
            //Event handling
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            //UPDATE
            self.dispatcher.dispatch(&self.robot_world);
            self.game_world.maintain();
            self.content_world.maintain();
            self.robot_world.maintain();

            //chiamare più volte il rendere per ogni tipo di cosa da renderizzare
            //
            self.canvas.clear();

            let _ = renderer::render(
                &mut self.canvas,
                &textures,
                self.game_world.system_data(),
                self.screen_offset,
            );
            let _ = renderer::render(
                &mut self.canvas,
                &textures,
                self.content_world.system_data(),
                self.screen_offset,
            );
            let _ = renderer::render(
                &mut self.canvas,
                &textures,
                self.robot_world.system_data(),
                self.screen_offset,
            );

            self.canvas.present();

            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }

    pub fn update_world(&mut self, world: Vec<Vec<Option<Tile>>>) {
        self.game_world.delete_all();
        self.content_world.delete_all();
        let mut y = 0;
        let mut x;

        for rows in world.iter() {
            x = 0;
            for cols in rows {
                match cols {
                    Some(t) => {
                        self.game_world
                            .create_entity()
                            .with(Position(
                                Point::new(x * TILE_SIZE, y * TILE_SIZE).scale(ZOOM_LEVEL),
                            ))
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
                                self.content_world
                                    .create_entity()
                                    .with(Position(
                                        Point::new(x * TILE_SIZE, y * TILE_SIZE).scale(ZOOM_LEVEL),
                                    ))
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
                    None => {}
                }
                x += 1;
            }
            y += 1;
        }
    }

    pub fn update_robot(
        &mut self,
        coords: Option<(usize, usize)>,
        last_coords: Option<(usize, usize)>,
    ) {
        //usare le coordinate per calcolare la direzione
        match coords {
            Some(coords) => {
                let last = last_coords.unwrap();
                let dir;
                if (coords.0 as i32 - last.0 as i32) > 0 {
                    dir = Direction::Down;
                } else if (coords.0 as i32 - last.0 as i32) > 0 {
                    dir = Direction::Up;
                } else if (coords.1 as i32 - last.1 as i32) > 0 {
                    dir = Direction::Right;
                } else {
                    dir = Direction::Left;
                }
                println!("GUII {:?}", dir.clone());

                self.robot_world.insert(Some(dir.clone()));
                //self.game_world.insert(dir.clone());
            }
            None => {}
        };
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
        let road_texture = self.texture_creator.load_texture(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("assets")
                .join("tiles")
                .join("street.png"),
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
        let _ = textures
            .add_texture(TextureType::Tile(TileType::Street), &road_texture)
            .clone();

        for _i in 0..TILE_SIZE {
            let mut event_pump = self.sdl_context.event_pump().unwrap();
            //Event handling
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {}
                    Event::MouseWheel { y: 1, .. } => {
                        //zoom in
                    }
                    Event::MouseWheel { y: -1, .. } => {
                        //zoom out
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Left),
                        repeat: false,
                        ..
                    } => {
                        self.screen_offset.0 += 10;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Right),
                        repeat: false,
                        ..
                    } => {
                        self.screen_offset.0 -= 10;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Down),
                        repeat: false,
                        ..
                    } => {
                        self.screen_offset.1 -= 10;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Up),
                        repeat: false,
                        ..
                    } => {
                        self.screen_offset.1 += 10;
                    }
                    Event::MouseMotion {
                        mousestate,
                        xrel,
                        yrel,
                        ..
                    } => {
                        if mousestate.right() {
                            self.screen_offset.0 += xrel;
                            self.screen_offset.1 += yrel;
                        }
                    }
                    _ => {}
                }
            }

            //UPDATE
            self.dispatcher.dispatch(&self.robot_world);
            self.game_world.maintain();
            self.content_world.maintain();
            self.robot_world.maintain();

            //chiamare più volte il rendere per ogni tipo di cosa da renderizzare
            //
            self.canvas.clear();

            let _ = renderer::render(
                &mut self.canvas,
                &textures,
                self.game_world.system_data(),
                self.screen_offset,
            );
            let _ = renderer::render(
                &mut self.canvas,
                &textures,
                self.content_world.system_data(),
                self.screen_offset,
            );
            let _ = renderer::render(
                &mut self.canvas,
                &textures,
                self.robot_world.system_data(),
                self.screen_offset,
            );

            self.canvas.present();

            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }
}
