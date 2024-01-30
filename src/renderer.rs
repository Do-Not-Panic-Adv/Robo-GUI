use crate::components::drawable_components::{Position, Sprite};
use crate::texture_manager::Textures;
use crate::{Camera, ZOOM_LEVEL};

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
    offset: (i32, i32),
    camera: &Camera,
) -> Result<(), String> {
    let (width, height) = canvas.output_size()?;

    //USE MARKER COMPONENT TO IMPLEMENT SEPARATE RENDERER FOR THE ROBOT AND TILES

    for (pos, sprite) in (&data.0, &data.1).join() {
        //Puts the (0,0) coordinate int the center
        let screen_position = pos.0 + Point::new(width as i32 / 2, height as i32 / 2);

        let scaled_width = sprite.region.width() as i32 + camera.zoom_level;
        let scaled_height = sprite.region.height() as i32 + camera.zoom_level;

        let mut screen_rect = Rect::from_center(
            screen_position.offset(offset.0, offset.1),
            scaled_width as u32,
            scaled_height as u32,
        );
        //println!("{:?} {:?}", screen_rect, screen_position);
        //canvas.set_scale(camera.zoom_level, camera.zoom_level)?;
        canvas.copy(&texture, sprite.region, screen_rect)?;
    }

    Ok(())
}
