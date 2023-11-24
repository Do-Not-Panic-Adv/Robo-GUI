use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};

use std::path::Path;
use std::time::Duration;

const HEIGHT: u32 = 600;
const WIDTH: u32 = 800;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Player {
    position: Point,
    sprite: Rect,
    direction: Option<Direction>,
    speed: i32,
}

pub fn init() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("big cock", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .expect("could not initialize window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not create canvas");

    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(Path::new("assets/reaper.png"))?;

    let mut player_list: Vec<Player> = vec![];

    let player1: Player = Player {
        position: Point::new(0, 0),
        sprite: Rect::new(0, 0, 32, 36),
        direction: None,
        speed: 5,
    };

    player_list.push(player1);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut bg = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
    'running: loop {
        //Event handling
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseMotion { x, y, .. } => {
                    bg = Color::RGB(
                        100,
                        ((x * 255) / WIDTH as i32) as u8,
                        ((y * 255) / HEIGHT as i32) as u8,
                    )
                }
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    repeat: false,
                    ..
                } => {
                    for player in player_list.iter_mut() {
                        player.direction = Some(Direction::Up)
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    repeat: false,
                    ..
                } => {
                    for player in player_list.iter_mut() {
                        player.direction = Some(Direction::Down)
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    repeat: false,
                    ..
                } => {
                    for player in player_list.iter_mut() {
                        player.direction = Some(Direction::Left)
                    }
                }

                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    repeat: false,
                    ..
                } => {
                    for player in player_list.iter_mut() {
                        player.direction = Some(Direction::Right)
                    }
                }
                Event::KeyUp {
                    keycode:
                        Some(Keycode::W) | Some(Keycode::A) | Some(Keycode::S) | Some(Keycode::D),
                    ..
                } => {
                    for player in player_list.iter_mut() {
                        //player.direction = None
                    }
                }

                _ => {}
            }
        }

        //UPDATE
        player_update(&mut player_list);
        render(&mut canvas, bg, &texture, &player_list)?;

        //Time mgmt
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}

fn player_update(player_list: &mut Vec<Player>) {
    for player in player_list.iter_mut() {
        match player.direction.as_mut() {
            Some(Direction::Up) => player.position += Point::new(0, -player.speed),
            Some(Direction::Down) => player.position += Point::new(0, player.speed),
            Some(Direction::Left) => player.position += Point::new(-player.speed, 0),

            Some(Direction::Right) => player.position += Point::new(player.speed, 0),
            None => (),
        }
    }
}
fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    player_list: &Vec<Player>,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    for player in player_list.iter() {
        let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(
            screen_position,
            player.sprite.width(),
            player.sprite.height(),
        );

        canvas.copy(texture, player.sprite, screen_rect)?;
    }
    canvas.present();

    Ok(())
}
