use std::{collections::HashMap, hash::Hash};

use robotics_lib::world::tile::{Content, TileType};
use sdl2::{rect::Rect, render::Texture};

use crate::TILE_SIZE;

pub(crate) struct Textures<'texture>(pub HashMap<TextureType, Vec<&'texture Texture<'texture>>>);

pub(crate) struct SpriteTable(pub HashMap<TextureType, Rect>);
impl SpriteTable {
    pub fn new() -> Self {
        SpriteTable(HashMap::new())
    }
    pub fn load_prites(&mut self) {
        self.0.insert(
            TextureType::Robot,
            Rect::new(TILE_SIZE * 2, 0, TILE_SIZE as u32, TILE_SIZE as u32),
        );
        self.0.insert(
            TextureType::Tile(TileType::Grass),
            Rect::new(
                TILE_SIZE * 0,
                TILE_SIZE * 0,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );

        self.0.insert(
            TextureType::Tile(TileType::Sand),
            Rect::new(
                TILE_SIZE * 5,
                TILE_SIZE * 0,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );

        self.0.insert(
            TextureType::Content(Content::Rock(0)),
            Rect::new(
                TILE_SIZE * 2,
                TILE_SIZE * 0,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );
        self.0.insert(
            TextureType::Content(Content::Tree(0)),
            Rect::new(
                TILE_SIZE * 0,
                TILE_SIZE * 1,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );

        self.0.insert(
            TextureType::Tile(TileType::Street),
            Rect::new(
                TILE_SIZE * 1,
                TILE_SIZE * 0,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );
        self.0.insert(
            TextureType::Tile(TileType::ShallowWater),
            Rect::new(
                TILE_SIZE * 6,
                TILE_SIZE * 0,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );
        self.0.insert(
            TextureType::Tile(TileType::DeepWater),
            Rect::new(
                TILE_SIZE * 7,
                TILE_SIZE * 0,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );

        self.0.insert(
            TextureType::Tile(TileType::Teleport(false)),
            Rect::new(
                TILE_SIZE * 2,
                TILE_SIZE * 1,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );
        self.0.insert(
            TextureType::Tile(TileType::Teleport(true)),
            Rect::new(
                TILE_SIZE * 2,
                TILE_SIZE * 2,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );

        self.0.insert(
            TextureType::Tile(TileType::Wall),
            Rect::new(
                TILE_SIZE * 3,
                TILE_SIZE * 1,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );
        self.0.insert(
            TextureType::Tile(TileType::Mountain),
            Rect::new(
                TILE_SIZE * 4,
                TILE_SIZE * 1,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );
        self.0.insert(
            TextureType::Tile(TileType::Snow),
            Rect::new(
                TILE_SIZE * 6,
                TILE_SIZE * 1,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );
        self.0.insert(
            TextureType::Tile(TileType::Lava),
            Rect::new(
                TILE_SIZE * 7,
                TILE_SIZE * 1,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );
        self.0.insert(
            TextureType::Tile(TileType::Hill),
            Rect::new(
                TILE_SIZE * 5,
                TILE_SIZE * 1,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );
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
