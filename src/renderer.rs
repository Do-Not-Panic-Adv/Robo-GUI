use std::ops::Div;

use crate::components::drawable_components::{Position, Sprite};
use crate::{Camera, TILE_SIZE};

use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use specs::prelude::*;
use specs::ReadStorage;

//this Extracts data from every entity that has a Position ans Sprite component
type SystemData<'a> = (ReadStorage<'a, Position>, ReadStorage<'a, Sprite>);

pub(crate) fn render(
    canvas: &mut WindowCanvas,
    texture: &Texture,
    data: SystemData,
    camera: &Camera,
) -> Result<(), String> {
    //USE MARKER COMPONENT TO IMPLEMENT SEPARATE RENDERER FOR THE ROBOT AND TILES

    for (pos, sprite) in (&data.0, &data.1).join() {
        //add check if the compomentent to be rendered is inside the viewport

        //this rappresents the point in the canvas where the sprite will be placed
        let screen_position = calculate_screen_position(pos.0, camera, canvas);

        let scaled_width = sprite.region.width() as i32 + camera.zoom_level;
        let scaled_height = sprite.region.height() as i32 + camera.zoom_level;

        //this represents the area of the screen on which the sprite region will be placed to.
        let screen_rect =
            Rect::from_center(screen_position, scaled_width as u32, scaled_height as u32);
        //TODO: Create a function that a screen takes screen_position, zoom_level and tilesize are return
        //the map coordinate and vice versa

        //println!( "{:?} {:?} {:?}", screen_position, pos.0.div(TILE_SIZE), calculate_map_coords(screen_position, camera, canvas).div(TILE_SIZE));
        canvas.copy(&texture, sprite.region, screen_rect)?;
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

    component_pos + Point::new(camera.screen_offset.0, camera.screen_offset.1) //mouse mov
        //- Point::new( (camera.zoom_level * TILE_SIZE) / 2, (camera.zoom_level * TILE_SIZE) / 2,)
        + Point::new(
            (camera.zoom_level * screen_position.x) / TILE_SIZE,
            (camera.zoom_level * screen_position.y) / TILE_SIZE,
        )
}
pub(crate) fn calculate_map_coords(
    screen_position: Point,
    camera: &Camera,
    canvas: &WindowCanvas,
) -> Point {
    let (window_width, window_height) = canvas.output_size().unwrap();
    let tmp = screen_position
        //- Point::new(window_width as i32 / 2, window_height as i32 / 2)
    - Point::new(camera.screen_offset.0, camera.screen_offset.1)
        + Point::new(
            (TILE_SIZE + camera.zoom_level) / 2,
            (TILE_SIZE + camera.zoom_level) / 2,
        );
    Point::new(
        tmp.x / (TILE_SIZE + camera.zoom_level),
        tmp.y / (TILE_SIZE + camera.zoom_level),
    )
}
