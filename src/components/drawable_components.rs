use sdl2::rect::{Point, Rect};
use specs::{Component, VecStorage};

use crate::texture_manager::TextureType;

#[derive(Debug)]
pub struct Position(pub(crate) Point);

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Sprite {
    pub(crate) region: Rect,
    pub(crate) texture_type: TextureType,
}

impl Component for Sprite {
    type Storage = VecStorage<Self>;
}
