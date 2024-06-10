use std::{collections::HashMap, hash::Hash};

use robotics_lib::world::{
    environmental_conditions::{DayTime, WeatherType},
    tile::{Content, TileType},
};
use sdl2::{pixels::Color, rect::Rect};

use crate::TILE_SIZE;

const FONT_STRING: &str =
    "!#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz{|}~ ";

#[derive(Debug)]
pub(crate) struct SpriteTable(pub HashMap<TextureType, Rect>);
impl SpriteTable {
    pub fn new() -> Self {
        SpriteTable(HashMap::new())
    }
    pub fn load_default_sprites(&mut self) {
        //TODO: add parsing from json???
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
            TextureType::Content(Content::Garbage(0)),
            Rect::new(
                TILE_SIZE * 4,
                TILE_SIZE * 2,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );
        self.0.insert(
            TextureType::Content(Content::Fire),
            Rect::new(
                TILE_SIZE * 5,
                TILE_SIZE * 2,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );
        self.0.insert(
            TextureType::Content(Content::Coin(0)),
            Rect::new(
                TILE_SIZE * 6,
                TILE_SIZE * 2,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );
        self.0.insert(
            TextureType::Content(Content::Bin(0..0)),
            Rect::new(
                TILE_SIZE * 7,
                TILE_SIZE * 2,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );
        self.0.insert(
            TextureType::Content(Content::Crate(0..0)),
            Rect::new(
                TILE_SIZE * 0,
                TILE_SIZE * 3,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );
        self.0.insert(
            TextureType::Content(Content::Bank(0..0)),
            Rect::new(
                TILE_SIZE * 1,
                TILE_SIZE * 3,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );
        self.0.insert(
            TextureType::Content(Content::Market(0)),
            Rect::new(
                TILE_SIZE * 2,
                TILE_SIZE * 3,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );
        self.0.insert(
            TextureType::Content(Content::Water(0)),
            Rect::new(
                TILE_SIZE * 3,
                TILE_SIZE * 3,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );
        self.0.insert(
            TextureType::Content(Content::Fish(0)),
            Rect::new(
                TILE_SIZE * 4,
                TILE_SIZE * 3,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );
        self.0.insert(
            TextureType::Content(Content::Building),
            Rect::new(
                TILE_SIZE * 5,
                TILE_SIZE * 3,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );
        self.0.insert(
            TextureType::Content(Content::Bush(0)),
            Rect::new(
                TILE_SIZE * 6,
                TILE_SIZE * 3,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );
        self.0.insert(
            TextureType::Content(Content::JollyBlock(0)),
            Rect::new(
                TILE_SIZE * 7,
                TILE_SIZE * 3,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );
        self.0.insert(
            TextureType::Content(Content::Scarecrow),
            Rect::new(
                TILE_SIZE * 0,
                TILE_SIZE * 4,
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
                TILE_SIZE * 5,
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
                TILE_SIZE * 4,
                TILE_SIZE * 1,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );

        self.0.insert(
            TextureType::Overlay(OverlayType::TileHover),
            Rect::new(
                TILE_SIZE * 0,
                TILE_SIZE * 6,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );
        self.0.insert(
            TextureType::Overlay(OverlayType::TileMarker),
            Rect::new(
                TILE_SIZE * 1,
                TILE_SIZE * 6,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );
        self.0.insert(
            TextureType::Time(DayTime::Morning),
            Rect::new(
                TILE_SIZE * 0,
                TILE_SIZE * 17,
                TILE_SIZE as u32 * 5,
                TILE_SIZE as u32 * 3,
            ),
        );
        self.0.insert(
            TextureType::Time(DayTime::Afternoon),
            Rect::new(
                TILE_SIZE * 5,
                TILE_SIZE * 17,
                TILE_SIZE as u32 * 5,
                TILE_SIZE as u32 * 3,
            ),
        );
        self.0.insert(
            TextureType::Time(DayTime::Night),
            Rect::new(
                TILE_SIZE * 10,
                TILE_SIZE * 17,
                TILE_SIZE as u32 * 5,
                TILE_SIZE as u32 * 3,
            ),
        );
        self.0.insert(
            TextureType::EnvCondition(WeatherType::Foggy),
            Rect::new(
                TILE_SIZE * 0,
                TILE_SIZE * 7,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );
        self.0.insert(
            TextureType::EnvCondition(WeatherType::Rainy),
            Rect::new(
                TILE_SIZE * 0,
                TILE_SIZE * 8,
                TILE_SIZE as u32 * 5,
                TILE_SIZE as u32 * 3,
            ),
        );
        self.0.insert(
            TextureType::EnvCondition(WeatherType::Sunny),
            Rect::new(
                TILE_SIZE * 3,
                TILE_SIZE * 7,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );
        self.0.insert(
            TextureType::EnvCondition(WeatherType::TrentinoSnow),
            Rect::new(
                TILE_SIZE * 5,
                TILE_SIZE * 8,
                TILE_SIZE as u32 * 5,
                TILE_SIZE as u32 * 3,
            ),
        );
        self.0.insert(
            TextureType::EnvCondition(WeatherType::TropicalMonsoon),
            Rect::new(
                TILE_SIZE * 3,
                TILE_SIZE * 7,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ),
        );
    }
    pub fn load_default_font(&mut self) {
        let mut x = 0;
        let mut y = TILE_SIZE * 11;

        for c in FONT_STRING.chars() {
            self.0.insert(
                TextureType::FontCharater(c, 1.0, true),
                Rect::new(x.clone(), y.clone(), TILE_SIZE as u32, TILE_SIZE as u32),
            );
            x += TILE_SIZE;
            if x >= TILE_SIZE * 16 {
                x = 0;
                y += TILE_SIZE;
            }
        }
    }

    //sovrascrive la sprite di un determinato tt
    pub fn load_sprite(&mut self, tt: TextureType, rect: Rect) {
        let _ = self.0.insert(tt, rect);
    }
}

#[derive(Debug, Clone)]
pub enum TextureType {
    Robot,
    Tile(TileType),
    Content(Content),
    Overlay(OverlayType),
    Time(DayTime),
    EnvCondition(WeatherType),
    FontCharater(char, f32, bool),
    Item(Box<TextureType>, f32, bool),
    Square((u32, u32), Color, bool, bool), // centered, fixed
}

#[derive(Clone, Debug, PartialEq)]
pub enum OverlayType {
    TileHover,
    TileMarker,
}

impl PartialEq for TextureType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Tile(l0), Self::Tile(r0)) => l0 == r0,
            (Self::Content(l0), Self::Content(r0)) => l0 == r0,
            (Self::Overlay(l0), Self::Overlay(r0)) => l0 == r0,
            (Self::Time(l0), Self::Time(r0)) => l0 == r0,
            (Self::EnvCondition(l0), Self::EnvCondition(r0)) => l0 == r0,
            (Self::FontCharater(l0, _, _), Self::FontCharater(r0, _, _)) => l0 == r0,
            (Self::Item(l0, _, _), Self::Item(r0, _, _)) => *l0.clone() == *r0.clone(),
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
