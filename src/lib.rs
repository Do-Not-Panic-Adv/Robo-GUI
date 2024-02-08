use components::drawable_components::{Position, Sprite};
use components::movement_components::Velocity;

use robotics_lib::interface::Direction;
use robotics_lib::world::tile::{Content, Tile, TileType};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::Sdl;
use systems::movement_systems::{ChangeDirectionSystem, MoveSystem};

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use specs::{Builder, Dispatcher, DispatcherBuilder, World, WorldExt};

use texture_manager::SpriteTable;

use std::path::Path;
use std::time::Duration;

use crate::texture_manager::TextureType;

use oxagaudiotool::OxAgAudioTool;

mod animation;
mod components;
mod renderer;
mod systems;
mod texture_manager;

const HEIGHT: u32 = 480;
const WIDTH: u32 = 800;

const TILE_SIZE: i32 = 32;

pub struct MainState<'window> {
    sdl_context: Sdl,
    //window: Window,
    canvas: Canvas<Window>,
    game_world: World,
    robot_world: World,
    content_world: World,
    dispatcher: Dispatcher<'window, 'window>,
    //texture: Texture<'window>,
    //Provare a creare una structure per salvare le texture con Rc<RefCell>>
    texture_creator: TextureCreator<WindowContext>,
    sprite_table: SpriteTable,
    camera: Camera,
}

struct Camera {
    screen_offset: (i32, i32),
    chase_robot: bool,
    zoom_level: i32,
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

        //robot entity

        //chiama i system relativi al robot
        let mut dispatcher = DispatcherBuilder::new()
            .with(ChangeDirectionSystem, "ChangeDir", &[])
            .with(MoveSystem, "Movement", &["ChangeDir"])
            .build();

        dispatcher.setup(&mut robot_world);

        let mut sprite_table = SpriteTable::new();

        //tutte le sprite sono definite in questo metodo, implementare metodo per sovrascrivere
        sprite_table.load_default_prites();

        let camera = Camera {
            screen_offset: (0, 0),
            chase_robot: true,
            zoom_level: 0,
        };
        Ok(MainState {
            sdl_context,
            canvas,
            game_world,
            robot_world,
            content_world,
            dispatcher,
            texture_creator,
            sprite_table,
            camera,
        })
    }
    pub fn add_robot(&mut self, pos_x: usize, pos_y: usize) {
        self.robot_world
            .create_entity()
            .with(Position(Point::new(
                TILE_SIZE * pos_x as i32,
                TILE_SIZE * pos_y as i32,
            )))
            .with(Velocity {
                speed: 1,
                direction: None,
            })
            .with(Sprite {
                region: Rect::new(0, TILE_SIZE * 2, TILE_SIZE as u32, TILE_SIZE as u32),
                texture_type: TextureType::Robot,
            })
            .build();
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
                            .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
                            .with(Sprite {
                                region: *self
                                    .sprite_table
                                    .0
                                    .get(&TextureType::Tile(t.tile_type))
                                    .unwrap(),
                                texture_type: TextureType::Tile(t.tile_type),
                            })
                            .build();
                        match &t.content {
                            Content::None => {}
                            Content::Rock(_) => {
                                self.content_world
                                    .create_entity()
                                    .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
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
                            Content::Tree(_) => {
                                self.content_world
                                    .create_entity()
                                    .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
                                    .with(Sprite {
                                        region: *self
                                            .sprite_table
                                            .0
                                            .get(&TextureType::Content(Content::Tree(0)))
                                            .unwrap(),
                                        texture_type: TextureType::Content(Content::Tree(0)),
                                    })
                                    .build();
                            }
                            Content::Garbage(_) => {
                                self.content_world
                                    .create_entity()
                                    .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
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
                            Content::Fire => {
                                self.content_world
                                    .create_entity()
                                    .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
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
                            Content::Coin(_) => {
                                self.content_world
                                    .create_entity()
                                    .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
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
                            Content::Bin(_) => {
                                self.content_world
                                    .create_entity()
                                    .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
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
                            Content::Crate(_) => {
                                self.content_world
                                    .create_entity()
                                    .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
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

                            Content::Bank(_) => {
                                self.content_world
                                    .create_entity()
                                    .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
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

                            Content::Water(_) => {
                                self.content_world
                                    .create_entity()
                                    .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
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

                            Content::Market(_) => {
                                self.content_world
                                    .create_entity()
                                    .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
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
                            Content::Fish(_) => {
                                self.content_world
                                    .create_entity()
                                    .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
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
                            Content::Building => {
                                self.content_world
                                    .create_entity()
                                    .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
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
                            Content::Bush(_) => {
                                self.content_world
                                    .create_entity()
                                    .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
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

                            Content::JollyBlock(_) => {
                                self.content_world
                                    .create_entity()
                                    .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
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
                            Content::Scarecrow => {
                                self.content_world
                                    .create_entity()
                                    .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
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
                    dir = Some(Direction::Down);
                } else if (coords.0 as i32 - last.0 as i32) > 0 {
                    dir = Some(Direction::Up);
                } else if (coords.1 as i32 - last.1 as i32) > 0 {
                    dir = Some(Direction::Right);
                } else if (coords.1 as i32 - last.1 as i32) < 0 {
                    dir = Some(Direction::Left);
                } else {
                    println!("Not moved");
                    dir = None
                }

                self.robot_world.insert(Some(dir.clone()));
                //self.game_world.insert(dir.clone());
            }
            None => {}
        };
    }

    pub fn tick(&mut self) -> Result<(), String> {
        //let mut textures = Textures::new();

        //let grass_texture = self.texture_creator.load_texture( Path::new(env!("CARGO_MANIFEST_DIR")) .join("assets") .join("tiles") .join("grass.png"),)?;

        //let _ = textures .add_texture(TextureType::Tile(TileType::Grass), &grass_texture) .clone();

        let texture = self.texture_creator.load_texture(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("assets")
                .join("texture.png"),
        )?;

        for _i in 0..TILE_SIZE {
            let mut event_pump = self.sdl_context.event_pump().unwrap();

            //Event handling
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        return Err("quit".to_string());
                    }
                    Event::MouseWheel { y: 1, .. } => self.camera.zoom_level += 1,
                    Event::MouseWheel { y: -1, .. } => self.camera.zoom_level -= 1,
                    Event::KeyDown {
                        keycode: Some(Keycode::Left),
                        repeat: false,
                        ..
                    } => {
                        self.camera.screen_offset.0 += TILE_SIZE;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Right),
                        repeat: false,
                        ..
                    } => {
                        self.camera.screen_offset.0 -= TILE_SIZE;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Down),
                        repeat: false,
                        ..
                    } => {
                        self.camera.screen_offset.1 -= TILE_SIZE;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Up),
                        repeat: false,
                        ..
                    } => {
                        self.camera.screen_offset.1 += TILE_SIZE;
                    }
                    Event::MouseMotion {
                        mousestate,
                        xrel,
                        yrel,
                        y,
                        x,
                        ..
                    } => {
                        if mousestate.right() {
                            self.camera.screen_offset.0 += xrel;
                            self.camera.screen_offset.1 += yrel;
                        }
                        println!(
                            "Pointing: {:?} z:{:?}",
                            self.get_coords_from_pos(Point::new(x, y)),
                            self.camera.zoom_level
                        )
                    }
                    _ => {}
                }
            }

            //UPDATE
            self.dispatcher.dispatch(&self.robot_world);

            self.game_world.maintain();
            self.content_world.maintain();
            self.robot_world.maintain();

            self.canvas.clear();

            //FARE UNA LISTA DI ELEMENTI DA RENDERIZZARE E ITERARE QUELLA
            // renderizza tiles
            let _ = renderer::render(
                &mut self.canvas,
                &texture,
                self.game_world.system_data(),
                &self.camera,
            );

            //renderizza content
            let _ = renderer::render(
                &mut self.canvas,
                &texture,
                self.content_world.system_data(),
                &self.camera,
            );

            //renderizza robot
            let _ = renderer::render(
                &mut self.canvas,
                &texture,
                self.robot_world.system_data(),
                &self.camera,
            );

            //aggiungere chiamate renderer per env_conditions

            self.canvas.present();

            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }
    pub fn get_coords_from_pos(&self, pos: Point) -> (i32, i32) {
        let point = renderer::calculate_map_coords(pos, &self.camera, &self.canvas);
        ((point.x()), (point.y()))
    }
}
