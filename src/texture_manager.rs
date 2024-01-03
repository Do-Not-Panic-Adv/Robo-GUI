use std::{collections::HashMap, hash::Hash};

use robotics_lib::world::tile::{Content, TileType};
use sdl2::{rect::Rect, render::Texture};

pub(crate) struct Textures<'texture>(pub HashMap<TextureType, Vec<&'texture Texture<'texture>>>);

pub(crate) struct SpriteTable(pub HashMap<TextureType, Rect>);
impl SpriteTable {
    pub fn new() -> Self {
        SpriteTable(HashMap::new())
    }
}

impl<'texture> Textures<'texture> {
    pub fn new() -> Textures<'texture> {
        Textures(HashMap::new())
    }
    pub(crate) fn add_texture(&mut self, texture_type: TextureType, texture: &'texture Texture) {
        self.0.insert(texture_type, vec![texture.clone()]);
    }
}

#[derive(Debug)]
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
