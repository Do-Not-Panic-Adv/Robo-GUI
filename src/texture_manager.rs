use std::hash::Hash;

use robotics_lib::world::tile::{Content, TileType};
use sdl2::render::Texture;

pub(crate) enum TextureType {
    Robot,
    Tile(TileType),
    Content(Content),
}

impl PartialEq for TextureType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Tile(l0), Self::Tile(r0)) => l0 == r0,
            (Self::Content(l0), Self::Content(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
impl Eq for TextureType {}
impl Hash for TextureType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}
