use crate::components::drawable_components::{Position, Sprite};
use crate::texture_manager::{TextureType, Textures};

use robotics_lib::world::tile::TileType;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use specs::prelude::*;
use specs::ReadStorage;

//this Extracts data from every entity that has a Position ans Sprite component
type SystemData<'a> = (ReadStorage<'a, Position>, ReadStorage<'a, Sprite>);

//const RENDER_ORDER: Vec<TextureType> = [TextureType::Tile(d)];

pub(crate) fn render(
    canvas: &mut WindowCanvas,
    textures: &Textures,
    data: SystemData,
) -> Result<(), String> {
    let (width, height) = canvas.output_size()?;

    for (pos, sprite) in (&data.0, &data.1).join() {
        //Puts the (0,0) coordinate int the center
        let screen_position = pos.0 + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(
            screen_position,
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
