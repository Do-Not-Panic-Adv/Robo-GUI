use crate::components::drawable_components::{Position, Sprite};
use crate::texture_manager::Textures;
use crate::ZOOM_LEVEL;

use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use specs::prelude::*;
use specs::ReadStorage;

//this Extracts data from every entity that has a Position ans Sprite component
type SystemData<'a> = (ReadStorage<'a, Position>, ReadStorage<'a, Sprite>);

pub(crate) fn render(
    canvas: &mut WindowCanvas,
    textures: &Textures,
    data: SystemData,
    offset: (i32, i32),
) -> Result<(), String> {
    let (width, height) = canvas.output_size()?;

    //USE MARKER COMPONENT TO IMPLEMENT SEPARATE RENDERER FOR THE ROBOT AND TILES

    for (pos, sprite) in (&data.0, &data.1).join() {
        //Puts the (0,0) coordinate int the center
        let screen_position = pos.0 + Point::new(width as i32 / 2, height as i32 / 2);
        //let screen_position = Point::new(0, 0);

        let screen_rect = Rect::from_center(
            screen_position.scale(ZOOM_LEVEL).offset(offset.0, offset.1),
            sprite.region.width(),
            sprite.region.height(),
        );
        canvas.copy(
            &textures.0.get(&sprite.texture_type).unwrap()[0],
            sprite.region,
            screen_rect,
        )?;
    }

    Ok(())
}
