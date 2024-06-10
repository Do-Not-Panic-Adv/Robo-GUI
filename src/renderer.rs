use crate::components::drawable_components::{Position, Sprite};
use crate::texture_manager::TextureType;
use crate::{Camera, HEIGHT, TILE_SIZE, WIDTH};

use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use specs::prelude::*;
use specs::ReadStorage;

//this Extracts data from every entity that has a Position ans Sprite component
type SystemData<'a> = (ReadStorage<'a, Position>, ReadStorage<'a, Sprite>);

pub(crate) fn render_sprites(
    canvas: &mut WindowCanvas,
    texture: &mut Texture,
    data: SystemData,
    camera: &mut Camera,
) -> Result<(), String> {
    for (pos, sprite) in (&data.0, &data.1).join() {
        //TODO: add check if the compomentent to be rendered is inside the viewport
        let (window_width, window_height) = canvas.output_size().unwrap();

        if camera.chase_robot && sprite.texture_type == TextureType::Robot {
            camera.robot_position = pos.0;
            camera.screen_offset = (window_width as i32 / 2, window_height as i32 / 2);
        }

        match sprite.texture_type {
            TextureType::Time(_) | TextureType::EnvCondition(_) => {
                canvas.copy(&texture, sprite.region, None)?
            }
            TextureType::FontCharater(_, scale, fixed) | TextureType::Item(_, scale, fixed) => {
                let screen_rect;
                if fixed {
                    screen_rect = Rect::from_center(
                        pos.0,
                        (sprite.region.width() as i32 + (TILE_SIZE as f32 * (scale - 1.0)) as i32)
                            as u32,
                        (sprite.region.height() as i32 + (TILE_SIZE as f32 * (scale - 1.0)) as i32)
                            as u32,
                    );
                } else {
                    let screen_position = calculate_screen_position(pos.0, camera, canvas);
                    screen_rect = Rect::from_center(
                        screen_position,
                        (sprite.region.width() as i32 + (TILE_SIZE as f32 * (scale - 1.0)) as i32)
                            as u32,
                        (sprite.region.height() as i32 + (TILE_SIZE as f32 * (scale - 1.0)) as i32)
                            as u32,
                    );
                }

                canvas.copy(&texture, sprite.region, screen_rect)?;
            }
            TextureType::Square(size, color, centered, fixed) => {
                let screen_rect;
                if fixed {
                    if centered {
                        screen_rect = Rect::from_center(
                            Point::new(WIDTH as i32 / 2, HEIGHT as i32 / 2),
                            size.0,
                            size.1,
                        );
                    } else {
                        screen_rect = Rect::from_center(pos.0, size.0, size.1);
                    }
                } else {
                    let screen_position = calculate_screen_position(pos.0, camera, canvas);
                    screen_rect = Rect::from_center(screen_position, size.0, size.1);
                }
                canvas.set_draw_color(color);
                canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
                canvas.fill_rect(screen_rect)?;
                canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
            }
            _ => {
                //this rappresents the point in the canvas where the sprite will be placed
                let screen_position = calculate_screen_position(pos.0, camera, canvas);

                let scaled_width = sprite.region.width() as i32 + camera.zoom_level;
                let scaled_height = sprite.region.height() as i32 + camera.zoom_level;

                //this represents the area of the screen on which the sprite region will be placed to.
                let screen_rect =
                    Rect::from_center(screen_position, scaled_width as u32, scaled_height as u32);

                canvas.copy(&texture, sprite.region, screen_rect)?;
            }
        }
    }

    Ok(())
}
pub(crate) fn calculate_screen_position(
    component_pos: Point,
    camera: &Camera,
    canvas: &WindowCanvas,
) -> Point {
    //TODO: add camera following
    let (window_width, window_height) = canvas.output_size().unwrap();
    let screen_position = component_pos; // Point::new(window_width as i32 / 2, window_height as i32 / 2);

    if camera.chase_robot {
        //doesn't work properly
        component_pos - Point::new(camera.robot_position.x(), camera.robot_position.y())
            + Point::new(camera.screen_offset.0, camera.screen_offset.1) //mouse mov
            + Point::new( camera.zoom_level
                    * (screen_position.x())
                    / TILE_SIZE,
                camera.zoom_level
                    * (screen_position.y())
                    / TILE_SIZE,
            )
    } else {
        component_pos //+ Point::new(camera.screen_offset.0, camera.screen_offset.1) //mouse mov
        + Point::new( camera.screen_offset.0 +
        camera.zoom_level * (screen_position.x()- ((window_width as i32/2)- camera.screen_offset.0))/ TILE_SIZE ,
        camera.screen_offset.1 +
        camera.zoom_level * (screen_position.y()- ((window_height as i32/2) - camera.screen_offset.1))/ TILE_SIZE ,
        )
    }
}
pub(crate) fn calculate_map_coords(
    screen_position: Point,
    camera: &Camera,
    canvas: &WindowCanvas,
) -> Point {
    let tmp;
    let (window_width, window_height) = canvas.output_size().unwrap();

    if camera.chase_robot {
        tmp = Point::new(
            (TILE_SIZE
                * (screen_position.x() - camera.screen_offset.0 + camera.robot_position.x())
                + camera.zoom_level
                    * (window_width as i32 / 2 - camera.screen_offset.0
                        + camera.robot_position.x()))
                / (camera.zoom_level + TILE_SIZE),
            (TILE_SIZE
                * (screen_position.y() - camera.screen_offset.1 + camera.robot_position.x())
                + camera.zoom_level
                    * (window_height as i32 / 2 - camera.screen_offset.1
                        + camera.robot_position.y()))
                / (camera.zoom_level + TILE_SIZE),
        ) + Point::new(
            (TILE_SIZE + camera.zoom_level) / 2,
            (TILE_SIZE + camera.zoom_level) / 2,
        )
    } else {
        tmp = Point::new(
            (TILE_SIZE * (screen_position.x() - camera.screen_offset.0)
                + camera.zoom_level * (window_width as i32 / 2 - camera.screen_offset.0))
                / (camera.zoom_level + TILE_SIZE),
            (TILE_SIZE * (screen_position.y() - camera.screen_offset.1)
                + camera.zoom_level * (window_height as i32 / 2 - camera.screen_offset.1))
                / (camera.zoom_level + TILE_SIZE),
        ) + Point::new(
            (TILE_SIZE + camera.zoom_level) / 2,
            (TILE_SIZE + camera.zoom_level) / 2,
        );
    }
    Point::new(tmp.x / (TILE_SIZE), tmp.y / (TILE_SIZE))
    //tmp
}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub(crate) enum Layer {
    Tiles,
    Content,
    Robot,
    Weather,
    OverlayHint,
    OverlayHover,
    Time,
    Ui(String, u32, u32), // layer, sublayer
}
pub(crate) const RENDER_ORDER: [Layer; 7] = [
    Layer::Tiles,
    Layer::Content,
    Layer::Robot,
    Layer::Weather,
    Layer::OverlayHint,
    Layer::OverlayHover,
    Layer::Time,
];
