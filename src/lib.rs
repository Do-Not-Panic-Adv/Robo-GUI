use components::drawable_components::{Position, Sprite};
use components::movement_components::Velocity;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use specs::{Builder, DispatcherBuilder, World, WorldExt};

use robotics_lib::interface::Direction;
use robotics_lib::runner::{Runnable, Runner};

use std::path::Path;
use std::time::Duration;

mod components;
mod renderer;
mod systems;

const HEIGHT: u32 = 600;
const WIDTH: u32 = 800;

pub fn init(run: &mut impl Runnable) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("ROBOTICS", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .expect("could not initialize window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not create canvas");

    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let texture_creator = canvas.texture_creator();
    let reaper_texture = texture_creator.load_texture(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("assets")
            .join("bardo.png"),
    )?;
    let grass_texture = texture_creator.load_texture(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("assets")
            .join("Texture")
            .join("TX Tileset Grass.png"),
    )?;

    let textures = &[reaper_texture, grass_texture];

    let mut world: specs::World = World::new();
    world.register::<Velocity>();
    world.register::<Position>();
    world.register::<Sprite>();

    //robot
    world
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

    //second robot
    world
        .create_entity()
        .with(Position(Point::new(0, 0)))
        .with(Velocity {
            speed: 1,
            direction: Direction::Right,
        })
        .with(Sprite {
            region: Rect::new(0, 0, 26, 39),
            sprite_type: components::drawable_components::SpriteType::Robot,
        })
        .build();

    //grass tile
    world
        .create_entity()
        .with(Position(Point::new(0, 0)))
        .with(Sprite {
            region: Rect::new(0, 0, 26, 39),
            sprite_type: components::drawable_components::SpriteType::Tile,
        })
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(systems::robot_system::MoveRobotSystem, "Movement", &[])
        .build();

    dispatcher.setup(&mut world);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
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
        dispatcher.dispatch(&mut world);
        world.maintain();

        //render_world(&mut canvas, &grass_texture, &tile_list)?;
        //render_robot(&mut canvas, bg, &reaper_texture, &player_list)?;
        renderer::render(&mut canvas, textures, world.system_data());

        println!(
            "Robotic robo pos: {} - {}",
            run.get_coordinate().get_row(),
            run.get_coordinate().get_col()
        );

        //Time mgmt
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}

//fn robot_update(player_list: &mut Vec<Player>) {
//    for player in player_list.iter_mut() {
//       match player.direction.as_mut() {
//          Some(Direction::Up) => player.position += Point::new(0, -player.speed),
//         Some(Direction::Down) => player.position += Point::new(0, player.speed),
//        Some(Direction::Left) => player.position += Point::new(-player.speed, 0),
//
//           Some(Direction::Right) => player.position += Point::new(player.speed, 0),
//          None => (),
//     }
//  }
//}

//fn render_robot(
//   canvas: &mut WindowCanvas,
//  color: Color,
// texture: &Texture,
//player_list: &Vec<Player>,
//) -> Result<(), String> {
//   canvas.set_draw_color(color);
//
//   let (width, height) = canvas.output_size()?;
//
//   for player in player_list.iter() {
//      let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);
//     let screen_rect = Rect::from_center(
//        screen_position,
//       player.sprite.width(),
//      player.sprite.height(),
// );
// canvas.copy(texture, player.sprite, screen_rect)?;
//}
//Ok(())
//}

//fn render_world(
//   canvas: &mut WindowCanvas,
//  texture: &Texture,
// tile_list: &Vec<Tile>,
//) -> Result<(), String> {
//   let (width, height) = canvas.output_size()?;
//
//   for tile in tile_list.iter() {
//      let screen_position = tile.position + Point::new(width as i32 / 2, height as i32 / 2);
//     let screen_rect =
//        Rect::from_center(screen_position, tile.sprite.width(), tile.sprite.height());
//
//       canvas.copy(texture, tile.sprite, screen_rect)?;
//  }
//
//   Ok(())
//}
