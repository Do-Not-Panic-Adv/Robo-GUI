use crate::components::drawable_components::{Position, Sprite};

use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use specs::prelude::*;
use specs::ReadStorage;

//this Extracts data from every entity that has a Position ans Sprite component
pub type SystemData<'a> = (ReadStorage<'a, Position>, ReadStorage<'a, Sprite>);

pub fn render(
    canvas: &mut WindowCanvas,
    textures: &[Texture],
    data: SystemData,
) -> Result<(), String> {
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    for (pos, sprite) in (&data.0, &data.1).join() {
        let current_texture = match sprite.sprite_type {
            crate::components::drawable_components::SpriteType::Robot => 0,
            crate::components::drawable_components::SpriteType::Tile => 1,
        };
        //Puts the (0,0) coordinate int the center
        let screen_position = pos.0 + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(
            screen_position,
            sprite.region.width(),
            sprite.region.height(),
        );
        canvas.copy(&textures[current_texture], sprite.region, screen_rect)?;
    }

    canvas.present();

    Ok(())
}
