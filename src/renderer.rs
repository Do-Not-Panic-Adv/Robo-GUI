use std::collections::HashMap;

use crate::components::drawable_components::{Position, Sprite, SpriteType};
use crate::texture_manager::TextureType;

use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use specs::prelude::*;
use specs::ReadStorage;

//this Extracts data from every entity that has a Position ans Sprite component
pub type SystemData<'a> = (ReadStorage<'a, Position>, ReadStorage<'a, Sprite>);

pub fn render(
    canvas: &mut WindowCanvas,
    textures: &HashMap<TextureType, Box<Vec<Texture>>>,
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
            &textures.get(&TextureType::Robot).unwrap()[0],
            sprite.region,
            screen_rect,
        )?;
    }

    Ok(())
}
