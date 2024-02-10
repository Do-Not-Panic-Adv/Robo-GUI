use components::drawable_components::{Position, Sprite};
use components::movement_components::Velocity;

use renderer::render;
use robotics_lib::interface::Direction;
use robotics_lib::world::tile::{Content, Tile};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::Sdl;
use systems::movement_systems::{ChangeDirectionSystem, MoveSystem};

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use specs::{Builder, Dispatcher, DispatcherBuilder, Entity, World, WorldExt};

use texture_manager::SpriteTable;

use std::path::Path;
use std::time::Duration;

use crate::markers::Marker;
use crate::texture_manager::{OverlayType, TextureType};

//use oxagaudiotool::OxAgAudioTool;

mod animation;
mod components;
mod markers;
mod renderer;
mod systems;
pub mod texture_manager;

const HEIGHT: u32 = 480;
const WIDTH: u32 = 800;

pub const TILE_SIZE: i32 = 32;
const ROBOT_SPEED: i32 = 1;

const ORD_TILES: usize = 0;
const ORD_CONTENT: usize = 1;
const ORD_OVERLAY_HOVER: usize = 2;
const ORD_OVERLAY_HINT: usize = 3;
const ORD_ROBOT: usize = 4;

pub struct MainState<'window> {
    sdl_context: Sdl,
    //window: Window,
    canvas: Canvas<Window>,
    worlds: Vec<World>,
    tiles_world: Vec<Vec<Option<Tile>>>,
    dispatcher: Dispatcher<'window, 'window>,
    //texture: Texture<'window>,
    //Provare a creare una structure per salvare le texture con Rc<RefCell>>
    texture_creator: TextureCreator<WindowContext>,
    sprite_table: SpriteTable,
    camera: Camera,
    markers: Vec<Marker>,
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
            .accelerated()
            .build()
            .expect("could not create canvas");

        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

        let texture_creator = canvas.texture_creator();

        //world per le time
        let mut game_world = World::new();
        game_world.register::<Position>();
        game_world.register::<Sprite>();

        //world per il robot e altre cose che si muovono con una velocità
        let mut robot_world = World::new();
        robot_world.register::<Velocity>();
        robot_world.register::<Position>();
        robot_world.register::<Sprite>();

        //world per i content
        let mut content_world = World::new();
        content_world.register::<Position>();
        content_world.register::<Sprite>();

        let mut overlay_world_hover = World::new();
        overlay_world_hover.register::<Position>();
        overlay_world_hover.register::<Sprite>();

        let mut overlay_world_markers = World::new();
        overlay_world_markers.register::<Position>();
        overlay_world_markers.register::<Sprite>();

        robot_world.insert(Some(Direction::Right));

        //chiama i system relativi al robot
        let mut dispatcher = DispatcherBuilder::new()
            .with(ChangeDirectionSystem, "ChangeDir", &[])
            .with(MoveSystem, "Movement", &["ChangeDir"])
            .build();

        dispatcher.setup(&mut robot_world);

        let mut sprite_table = SpriteTable::new();

        //tutte le sprite sono definite in questo metodo, implementare metodo per sovrascrivere
        sprite_table.load_default_prites();
        //sprite_table.load_sprite( TextureType::Tile(TileType::Grass), Rect::new(12, 43, 60, 34),);

        let camera = Camera {
            screen_offset: (0, 0),
            chase_robot: true,
            zoom_level: 0,
        };

        let mut worlds: Vec<World> = Vec::new();
        worlds.push(game_world);
        worlds.push(content_world);
        worlds.push(overlay_world_hover);
        worlds.push(overlay_world_markers);
        worlds.push(robot_world);

        Ok(MainState {
            sdl_context,
            canvas,
            worlds,
            dispatcher,
            texture_creator,
            sprite_table,
            camera,
            tiles_world: Vec::new(),
            markers: Vec::new(),
        })
    }
    pub fn add_robot(&mut self, pos_x: usize, pos_y: usize) {
        self.worlds
            .get_mut(ORD_ROBOT)
            .unwrap()
            .create_entity()
            .with(Position(Point::new(
                TILE_SIZE * pos_x as i32,
                TILE_SIZE * pos_y as i32,
            )))
            .with(Velocity {
                speed: ROBOT_SPEED,
                direction: None,
            })
            .with(Sprite {
                region: Rect::new(0, TILE_SIZE * 2, TILE_SIZE as u32, TILE_SIZE as u32),
                texture_type: TextureType::Robot,
            })
            .build();
    }

    pub fn update_world(&mut self, world: Vec<Vec<Option<Tile>>>) {
        self.worlds.get_mut(ORD_TILES).unwrap().delete_all();
        self.worlds.get_mut(ORD_CONTENT).unwrap().delete_all();

        self.tiles_world = world.clone();

        let mut y = 0;
        let mut x;

        //cercare di sistemare sto schifo, aggiungere controlli per evitare di aggiungere entità
        //che sono fuori dalla viewport
        for rows in world.iter() {
            x = 0;
            for cols in rows {
                match cols {
                    Some(t) => {
                        self.worlds
                            .get_mut(ORD_TILES)
                            .unwrap()
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
                                self.worlds
                                    .get_mut(ORD_CONTENT)
                                    .unwrap()
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
                                self.worlds
                                    .get_mut(ORD_CONTENT)
                                    .unwrap()
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
                                self.worlds
                                    .get_mut(ORD_CONTENT)
                                    .unwrap()
                                    .create_entity()
                                    .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
                                    .with(Sprite {
                                        region: *self
                                            .sprite_table
                                            .0
                                            .get(&TextureType::Content(Content::Garbage(0)))
                                            .unwrap(),
                                        texture_type: TextureType::Content(Content::Garbage(0)),
                                    })
                                    .build();
                            }
                            Content::Fire => {
                                self.worlds
                                    .get_mut(ORD_CONTENT)
                                    .unwrap()
                                    .create_entity()
                                    .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
                                    .with(Sprite {
                                        region: *self
                                            .sprite_table
                                            .0
                                            .get(&TextureType::Content(Content::Fire))
                                            .unwrap(),
                                        texture_type: TextureType::Content(Content::Fire),
                                    })
                                    .build();
                            }
                            Content::Coin(_) => {
                                self.worlds
                                    .get_mut(ORD_CONTENT)
                                    .unwrap()
                                    .create_entity()
                                    .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
                                    .with(Sprite {
                                        region: *self
                                            .sprite_table
                                            .0
                                            .get(&TextureType::Content(Content::Coin(0)))
                                            .unwrap(),
                                        texture_type: TextureType::Content(Content::Coin(0)),
                                    })
                                    .build();
                            }
                            Content::Bin(_) => {
                                self.worlds
                                    .get_mut(ORD_CONTENT)
                                    .unwrap()
                                    .create_entity()
                                    .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
                                    .with(Sprite {
                                        region: *self
                                            .sprite_table
                                            .0
                                            .get(&TextureType::Content(Content::Bin(0..0)))
                                            .unwrap(),
                                        texture_type: TextureType::Content(Content::Bin(0..0)),
                                    })
                                    .build();
                            }
                            Content::Crate(_) => {
                                self.worlds
                                    .get_mut(ORD_CONTENT)
                                    .unwrap()
                                    .create_entity()
                                    .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
                                    .with(Sprite {
                                        region: *self
                                            .sprite_table
                                            .0
                                            .get(&TextureType::Content(Content::Crate(0..0)))
                                            .unwrap(),
                                        texture_type: TextureType::Content(Content::Crate(0..0)),
                                    })
                                    .build();
                            }

                            Content::Bank(_) => {
                                self.worlds
                                    .get_mut(ORD_CONTENT)
                                    .unwrap()
                                    .create_entity()
                                    .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
                                    .with(Sprite {
                                        region: *self
                                            .sprite_table
                                            .0
                                            .get(&TextureType::Content(Content::Bank(0..0)))
                                            .unwrap(),
                                        texture_type: TextureType::Content(Content::Bank(0..0)),
                                    })
                                    .build();
                            }

                            Content::Water(_) => {
                                self.worlds
                                    .get_mut(ORD_CONTENT)
                                    .unwrap()
                                    .create_entity()
                                    .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
                                    .with(Sprite {
                                        region: *self
                                            .sprite_table
                                            .0
                                            .get(&TextureType::Content(Content::Water(0)))
                                            .unwrap(),
                                        texture_type: TextureType::Content(Content::Water(0)),
                                    })
                                    .build();
                            }

                            Content::Market(_) => {
                                self.worlds
                                    .get_mut(ORD_CONTENT)
                                    .unwrap()
                                    .create_entity()
                                    .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
                                    .with(Sprite {
                                        region: *self
                                            .sprite_table
                                            .0
                                            .get(&TextureType::Content(Content::Market(0)))
                                            .unwrap(),
                                        texture_type: TextureType::Content(Content::Market(0)),
                                    })
                                    .build();
                            }
                            Content::Fish(_) => {
                                self.worlds
                                    .get_mut(ORD_CONTENT)
                                    .unwrap()
                                    .create_entity()
                                    .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
                                    .with(Sprite {
                                        region: *self
                                            .sprite_table
                                            .0
                                            .get(&TextureType::Content(Content::Fish(0)))
                                            .unwrap(),
                                        texture_type: TextureType::Content(Content::Fish(0)),
                                    })
                                    .build();
                            }
                            Content::Building => {
                                self.worlds
                                    .get_mut(ORD_CONTENT)
                                    .unwrap()
                                    .create_entity()
                                    .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
                                    .with(Sprite {
                                        region: *self
                                            .sprite_table
                                            .0
                                            .get(&TextureType::Content(Content::Building))
                                            .unwrap(),
                                        texture_type: TextureType::Content(Content::Building),
                                    })
                                    .build();
                            }
                            Content::Bush(_) => {
                                self.worlds
                                    .get_mut(ORD_CONTENT)
                                    .unwrap()
                                    .create_entity()
                                    .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
                                    .with(Sprite {
                                        region: *self
                                            .sprite_table
                                            .0
                                            .get(&TextureType::Content(Content::Bush(0)))
                                            .unwrap(),
                                        texture_type: TextureType::Content(Content::Bush(0)),
                                    })
                                    .build();
                            }

                            Content::JollyBlock(_) => {
                                self.worlds
                                    .get_mut(ORD_CONTENT)
                                    .unwrap()
                                    .create_entity()
                                    .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
                                    .with(Sprite {
                                        region: *self
                                            .sprite_table
                                            .0
                                            .get(&TextureType::Content(Content::JollyBlock(0)))
                                            .unwrap(),
                                        texture_type: TextureType::Content(Content::JollyBlock(0)),
                                    })
                                    .build();
                            }
                            Content::Scarecrow => {
                                self.worlds
                                    .get_mut(ORD_CONTENT)
                                    .unwrap()
                                    .create_entity()
                                    .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
                                    .with(Sprite {
                                        region: *self
                                            .sprite_table
                                            .0
                                            .get(&TextureType::Content(Content::Scarecrow))
                                            .unwrap(),
                                        texture_type: TextureType::Content(Content::Scarecrow),
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

                self.worlds
                    .get_mut(ORD_ROBOT)
                    .unwrap()
                    .insert(Some(dir.clone()));
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

        for _i in 0..(TILE_SIZE / ROBOT_SPEED) {
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
                    Event::MouseButtonDown {
                        mouse_btn, x, y, ..
                    } => match mouse_btn {
                        sdl2::mouse::MouseButton::Middle => {
                            let pos = self.get_coords_from_pos(Point::new(x, y));
                            self.markers.push(Marker::new(pos.0, pos.1));

                            self.worlds.get_mut(ORD_OVERLAY_HINT).unwrap().delete_all();

                            for marker in &self.markers {
                                self.worlds
                                    .get_mut(ORD_OVERLAY_HINT)
                                    .unwrap()
                                    .create_entity()
                                    .with(Position(Point::new(
                                        marker.get_pos().1 as i32 * TILE_SIZE,
                                        marker.get_pos().0 as i32 * TILE_SIZE,
                                    )))
                                    .with(Sprite {
                                        region: *self
                                            .sprite_table
                                            .0
                                            .get(&TextureType::Overlay(OverlayType::TileMarker))
                                            .unwrap(),
                                        texture_type: TextureType::Overlay(OverlayType::TileMarker),
                                    })
                                    .build();
                            }
                        }
                        _ => {}
                    },
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
                        let pos = self.get_coords_from_pos(Point::new(x, y));
                        //println!( "Pointing: {:?} z:{:?}, camera offset: {:?}", pos, self.camera.zoom_level, self.camera.screen_offset);
                        if self.tiles_world.len() > pos.1 as usize
                            && self.tiles_world[0].len() > pos.0 as usize
                        {
                            self.worlds.get_mut(ORD_OVERLAY_HOVER).unwrap().delete_all();
                            self.worlds
                                .get_mut(ORD_OVERLAY_HOVER)
                                .unwrap()
                                .create_entity()
                                .with(Position(Point::new(pos.0 * TILE_SIZE, pos.1 * TILE_SIZE)))
                                .with(Sprite {
                                    region: *self
                                        .sprite_table
                                        .0
                                        .get(&TextureType::Overlay(OverlayType::TileHover))
                                        .unwrap(),
                                    texture_type: TextureType::Overlay(OverlayType::TileHover),
                                })
                                .build();
                            println!(
                                "Pointing tile {:?}",
                                self.tiles_world[pos.1 as usize][pos.0 as usize]
                            )
                        }
                    }
                    _ => {}
                }
            }

            //UPDATE
            self.dispatcher
                .dispatch(&self.worlds.get_mut(ORD_ROBOT).unwrap());

            self.worlds.get_mut(ORD_TILES).unwrap().maintain();
            self.worlds.get_mut(ORD_CONTENT).unwrap().maintain();
            self.worlds.get_mut(ORD_ROBOT).unwrap().maintain();

            self.canvas.clear();

            for world in self.worlds.iter_mut() {
                let _ = render(
                    &mut self.canvas,
                    &texture,
                    world.system_data(),
                    &self.camera,
                );
            }

            //aggiungere chiamate renderer per env_conditions

            self.canvas.present();

            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }
    //possibilmente cambiare la struct Point con tipi buildin per evitare di importare sdl nel main
    pub fn get_coords_from_pos(&self, pos: Point) -> (i32, i32) {
        let point = renderer::calculate_map_coords(pos, &self.camera, &self.canvas);
        ((point.x()), (point.y()))
    }
    pub fn load_sprite(&mut self, tt: TextureType, x: i32, y: i32, height: u32, width: u32) {
        self.sprite_table
            .load_sprite(tt, Rect::new(x, y, width, height));
    }
    pub fn get_markers(&self) -> Vec<Marker> {
        self.markers.clone()
    }
    //TODO: implement deletion of markers
}
