use components::drawable_components::{Position, Sprite};
use components::movement_components::Velocity;

use markers::Markers;
use renderer::{calculate_map_coords, render};
use robotics_lib::interface::Direction;
use robotics_lib::world::environmental_conditions::{DayTime, WeatherType};
use robotics_lib::world::tile::{Content, Tile};
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

use crate::markers::Marker;
use crate::texture_manager::{OverlayType, TextureType};

//use oxagaudiotool::OxAgAudioTool;

mod animation;
mod components;
mod markers;
mod renderer;
mod systems;
pub mod texture_manager;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 480;

pub const TILE_SIZE: i32 = 32;
//const ROBOT_SPEED: i32 = 6;

const ORD_TILES: usize = 0;
const ORD_CONTENT: usize = 1;
const ORD_ROBOT: usize = 2;
const ORD_WEATHER: usize = 3;
const ORD_TIME: usize = 4;
const ORD_OVERLAY_HINT: usize = 5;
const ORD_OVERLAY_HOVER: usize = 6;

pub struct MainState<'window> {
    sdl_context: Sdl,
    //window: Window,
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    worlds: Vec<World>,
    tiles_world: Vec<Vec<Option<Tile>>>,
    dispatcher: Dispatcher<'window, 'window>,
    //texture: Texture<'window>,
    //Provare a creare una structure per salvare le texture con Rc<RefCell>>
    sprite_table: SpriteTable,
    camera: Camera,
    markers: Markers,
    robot_speed: i32,
    framerate: u32,
}

#[derive(Debug)]
struct Camera {
    screen_offset: (i32, i32),
    chase_robot: bool,
    zoom_level: i32,
    robot_position: Point,
}

impl<'window> MainState<'window> {
    pub fn new(robot_speed: i32) -> Result<MainState<'window>, String> {
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

        let mut weather_world = World::new();
        weather_world.register::<Position>();
        weather_world.register::<Sprite>();
        let mut time_world = World::new();
        time_world.register::<Position>();
        time_world.register::<Sprite>();

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
            chase_robot: false,
            zoom_level: 0,
            robot_position: Point::new(0, 0),
        };

        let mut worlds: Vec<World> = Vec::new();
        worlds.push(game_world);
        worlds.push(content_world);
        worlds.push(robot_world);
        worlds.push(weather_world);
        worlds.push(time_world);
        worlds.push(overlay_world_markers);
        worlds.push(overlay_world_hover);

        if robot_speed > 6 || robot_speed < 1 {
            return Err("speed has to be <= 6 and >=1".to_string());
        }

        Ok(MainState {
            sdl_context,
            canvas,
            worlds,
            dispatcher,
            texture_creator,
            sprite_table,
            camera,
            tiles_world: Vec::new(),
            markers: Markers::new(),
            robot_speed,
            framerate: 60,
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
                speed: 2_i32.pow(self.robot_speed as u32 - 1),
                direction: None,
            })
            .with(Sprite {
                region: Rect::new(0, TILE_SIZE * 2, TILE_SIZE as u32, TILE_SIZE as u32),
                texture_type: TextureType::Robot,
            })
            .build();

        // moves the camera relative to the start position of the robot
        self.camera.screen_offset.0 =
            -1 * pos_x as i32 * TILE_SIZE + self.canvas.output_size().unwrap().0 as i32 / 2;
        self.camera.screen_offset.1 =
            -1 * pos_y as i32 * TILE_SIZE + self.canvas.output_size().unwrap().1 as i32 / 2;
    }

    pub fn update_world(&mut self, world: Vec<Vec<Option<Tile>>>) {
        self.worlds.get_mut(ORD_TILES).unwrap().delete_all();
        self.worlds.get_mut(ORD_CONTENT).unwrap().delete_all();

        self.tiles_world = world.clone();

        let mut y = 0;
        let mut x;

        //let min_coords = calculate_map_coords(Point::new(0, 0), &self.camera, &self.canvas);
        let max_coords = calculate_map_coords(
            Point::new(WIDTH as i32, HEIGHT as i32),
            &self.camera,
            &self.canvas,
        );
        //if y < min_coords.y() / 2 {
        //   y = min_coords.y() / 2
        //}
        for rows in world.iter() {
            x = 0;

            if y > max_coords.y() * 2 {
                break;
            }
            //   if x < min_coords.x() / 2 {
            //      x = min_coords.x() / 2
            // }

            for cols in rows {
                if x > max_coords.x() * 2 {
                    break;
                }

                match cols {
                    Some(t) => {
                        MainState::add_drawable(
                            &mut self.worlds,
                            &self.sprite_table,
                            ORD_TILES,
                            TextureType::Tile(t.tile_type),
                            x,
                            y,
                        );
                        match &t.content {
                            Content::None => {}
                            Content::Rock(_) => {
                                MainState::add_drawable(
                                    &mut self.worlds,
                                    &self.sprite_table,
                                    ORD_CONTENT,
                                    TextureType::Content(Content::Rock(0)),
                                    x,
                                    y,
                                );
                            }
                            Content::Tree(_) => {
                                MainState::add_drawable(
                                    &mut self.worlds,
                                    &self.sprite_table,
                                    ORD_CONTENT,
                                    TextureType::Content(Content::Tree(0)),
                                    x,
                                    y,
                                );
                            }
                            Content::Garbage(_) => {
                                MainState::add_drawable(
                                    &mut self.worlds,
                                    &self.sprite_table,
                                    ORD_CONTENT,
                                    TextureType::Content(Content::Garbage(0)),
                                    x,
                                    y,
                                );
                            }
                            Content::Fire => {
                                MainState::add_drawable(
                                    &mut self.worlds,
                                    &self.sprite_table,
                                    ORD_CONTENT,
                                    TextureType::Content(Content::Fire),
                                    x,
                                    y,
                                );
                            }
                            Content::Coin(_) => {
                                MainState::add_drawable(
                                    &mut self.worlds,
                                    &self.sprite_table,
                                    ORD_CONTENT,
                                    TextureType::Content(Content::Coin(0)),
                                    x,
                                    y,
                                );
                            }
                            Content::Bin(_) => {
                                MainState::add_drawable(
                                    &mut self.worlds,
                                    &self.sprite_table,
                                    ORD_CONTENT,
                                    TextureType::Content(Content::Bin(0..0)),
                                    x,
                                    y,
                                );
                            }
                            Content::Crate(_) => {
                                MainState::add_drawable(
                                    &mut self.worlds,
                                    &self.sprite_table,
                                    ORD_CONTENT,
                                    TextureType::Content(Content::Crate(0..0)),
                                    x,
                                    y,
                                );
                            }

                            Content::Bank(_) => {
                                MainState::add_drawable(
                                    &mut self.worlds,
                                    &self.sprite_table,
                                    ORD_CONTENT,
                                    TextureType::Content(Content::Bank(0..0)),
                                    x,
                                    y,
                                );
                            }

                            Content::Water(_) => {
                                MainState::add_drawable(
                                    &mut self.worlds,
                                    &self.sprite_table,
                                    ORD_CONTENT,
                                    TextureType::Content(Content::Water(0)),
                                    x,
                                    y,
                                );
                            }

                            Content::Market(_) => {
                                MainState::add_drawable(
                                    &mut self.worlds,
                                    &self.sprite_table,
                                    ORD_CONTENT,
                                    TextureType::Content(Content::Market(0)),
                                    x,
                                    y,
                                );
                            }
                            Content::Fish(_) => {
                                MainState::add_drawable(
                                    &mut self.worlds,
                                    &self.sprite_table,
                                    ORD_CONTENT,
                                    TextureType::Content(Content::Fish(0)),
                                    x,
                                    y,
                                );
                            }
                            Content::Building => {
                                MainState::add_drawable(
                                    &mut self.worlds,
                                    &self.sprite_table,
                                    ORD_CONTENT,
                                    TextureType::Content(Content::Building),
                                    x,
                                    y,
                                );
                            }
                            Content::Bush(_) => {
                                MainState::add_drawable(
                                    &mut self.worlds,
                                    &self.sprite_table,
                                    ORD_CONTENT,
                                    TextureType::Content(Content::Bush(0)),
                                    x,
                                    y,
                                );
                            }

                            Content::JollyBlock(_) => {
                                MainState::add_drawable(
                                    &mut self.worlds,
                                    &self.sprite_table,
                                    ORD_CONTENT,
                                    TextureType::Content(Content::JollyBlock(0)),
                                    x,
                                    y,
                                );
                            }
                            Content::Scarecrow => {
                                MainState::add_drawable(
                                    &mut self.worlds,
                                    &self.sprite_table,
                                    ORD_CONTENT,
                                    TextureType::Content(Content::Scarecrow),
                                    x,
                                    y,
                                );
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
                } else if (coords.0 as i32 - last.0 as i32) < 0 {
                    dir = Some(Direction::Up);
                } else if (coords.1 as i32 - last.1 as i32) > 0 {
                    dir = Some(Direction::Right);
                } else if (coords.1 as i32 - last.1 as i32) < 0 {
                    dir = Some(Direction::Left);
                } else {
                    //println!("Not moved");
                    dir = None
                }

                self.worlds
                    .get_mut(ORD_ROBOT)
                    .unwrap()
                    .insert(Some(dir.clone()));
            }
            None => {}
        };
    }
    pub fn update_time_of_day(&mut self, time: DayTime) {
        let limits = self.get_drawable_indexes();
        self.worlds.get_mut(ORD_TIME).unwrap().delete_all();
        //for x in limits.0.x()..limits.1.x() {
        //   for y in limits.0.y()..limits.1.y() {
        MainState::add_drawable(
            &mut self.worlds,
            &self.sprite_table,
            ORD_TIME,
            TextureType::Time(time),
            0,
            0,
        );
        // }
        // }
    }
    pub fn update_weather(&mut self, w: WeatherType) {
        self.worlds.get_mut(ORD_WEATHER).unwrap().delete_all();
        MainState::add_drawable(
            &mut self.worlds,
            &self.sprite_table,
            ORD_WEATHER,
            TextureType::EnvCondition(w),
            0,
            0,
        );
    }

    /// Returns the get drawable indexes of this [`MainState`].
    //min e max
    fn get_drawable_indexes(&mut self) -> (Point, Point) {
        let (window_width, window_height) = self.canvas.output_size().unwrap();
        let min_coords = calculate_map_coords(Point::new(0, 0), &self.camera, &self.canvas);
        let max_coords = calculate_map_coords(
            Point::new(window_width as i32, window_height as i32),
            &self.camera,
            &self.canvas,
        );
        (min_coords, max_coords)
    }

    fn robot_stop(&mut self) {
        self.worlds
            .get_mut(ORD_ROBOT)
            .unwrap()
            .insert(Some(None::<Direction>));
    }

    pub fn tick(&mut self) -> Result<(), String> {
        //let mut textures = Textures::new();

        //let grass_texture = self.texture_creator.load_texture( Path::new(env!("CARGO_MANIFEST_DIR")) .join("assets") .join("tiles") .join("grass.png"),)?;

        //let _ = textures .add_texture(TextureType::Tile(TileType::Grass), &grass_texture) .clone();

        let mut texture = self.texture_creator.load_texture(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("assets")
                .join("texture.png"),
        )?;

        for _i in 0..(TILE_SIZE / 2_i32.pow(self.robot_speed as u32 - 1)) {
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
                    Event::MouseWheel { y: 1, .. } => {
                        self.camera.zoom_level += 1;
                    }
                    Event::MouseWheel { y: -1, .. } => {
                        if self.camera.zoom_level > -31 {
                            self.camera.zoom_level -= 1;
                        }
                    }
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
                    Event::KeyDown {
                        keycode: Some(Keycode::R),
                        repeat: false,
                        ..
                    } => {
                        self.camera.screen_offset = (0, 0);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Space),
                        repeat: false,
                        ..
                    } => {
                        let (window_width, window_height) = self.canvas.output_size().unwrap();
                        self.camera.chase_robot = !self.camera.chase_robot;
                        self.camera.screen_offset = (
                            -self.camera.robot_position.x() + window_width as i32 / 2,
                            -self.camera.robot_position.y() + window_height as i32 / 2,
                        );
                        self.camera.zoom_level = 0
                    }
                    Event::MouseButtonDown {
                        mouse_btn, x, y, ..
                    } => match mouse_btn {
                        sdl2::mouse::MouseButton::Middle => {
                            let pos = self.get_coords_from_pos(Point::new(x, y));
                            self.markers.toggle(pos);

                            self.worlds.get_mut(ORD_OVERLAY_HINT).unwrap().delete_all();

                            for marker in &self.markers.get_all() {
                                MainState::add_drawable(
                                    &mut self.worlds,
                                    &self.sprite_table,
                                    ORD_OVERLAY_HINT,
                                    TextureType::Overlay(OverlayType::TileMarker),
                                    marker.0 .1,
                                    marker.0 .0,
                                );
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
                        // println!( "Pointing: {:?} z:{:?}, camera offset: {:?}", pos, self.camera.zoom_level, self.camera.screen_offset);
                        if self.tiles_world.len() > pos.1 as usize
                            && self.tiles_world[0].len() > pos.0 as usize
                        {
                            self.worlds.get_mut(ORD_OVERLAY_HOVER).unwrap().delete_all();

                            MainState::add_drawable(
                                &mut self.worlds,
                                &self.sprite_table,
                                ORD_OVERLAY_HOVER,
                                TextureType::Overlay(OverlayType::TileHover),
                                pos.0,
                                pos.1,
                            );
                            //println!( "Pointing tile {:?}", self.tiles_world[pos.1 as usize][pos.0 as usize])
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
                    &mut texture,
                    world.system_data(),
                    &mut self.camera,
                );
            }

            //aggiungere chiamate renderer per env_conditions

            self.canvas.present();

            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / self.framerate));
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
    pub fn get_markers(&self) -> Vec<((i32, i32), Marker)> {
        self.markers.get_all()
    }
    //TODO: implement deletion of markers

    pub fn set_framerate(&mut self, framerate: u32) {
        self.framerate = framerate
    }
    pub(crate) fn add_drawable(
        worlds: &mut Vec<World>,
        sp: &SpriteTable,
        ord: usize,
        tt: TextureType,
        x: i32,
        y: i32,
    ) {
        worlds
            .get_mut(ord)
            .unwrap()
            .create_entity()
            .with(Position(Point::new(x * TILE_SIZE, y * TILE_SIZE)))
            .with(Sprite {
                region: *sp.0.get(&tt).unwrap(),
                texture_type: tt,
            })
            .build();
    }
}
