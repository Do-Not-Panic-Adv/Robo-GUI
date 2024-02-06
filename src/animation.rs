use sdl2::render::Texture;

use crate::texture_manager::SpriteTable;

pub(crate) struct Animation<'a> {
    texture: Texture<'a>,
    sprites: SpriteTable,
}
