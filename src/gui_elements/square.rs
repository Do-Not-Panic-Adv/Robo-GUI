use sdl2::pixels::Color;

use crate::{texture_manager::TextureType, MainState, ORD_UI};

use super::draw::Drawable;

pub(crate) struct Square {
    position: (i32, i32),
    size: (u32, u32),
    fixed: bool,
    centered: bool,
    color: Color,
}

impl Square {
    pub fn new(
        position: (i32, i32),
        size: (u32, u32),
        fixed: bool,
        centered: bool,
        color: Color,
    ) -> Self {
        Self {
            position,
            size,
            fixed,
            centered,
            color,
        }
    }
    pub fn set_position(&mut self, position: (i32, i32)) {
        self.position = position;
    }
    pub fn get_position(&self) -> (i32, i32) {
        self.position
    }
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    pub fn get_color(&self) -> Color {
        self.color
    }
    pub fn get_fixed(&self) -> bool {
        self.fixed
    }

    fn get_centered(&self) -> bool {
        self.centered
    }
    fn get_size(&self) -> (u32, u32) {
        self.size
    }
}

impl Drawable for Square {
    fn draw(&self, state: &mut MainState) {
        MainState::add_drawable(
            &mut state.worlds,
            &state.sprite_table,
            ORD_UI,
            TextureType::Square(
                self.get_size(),
                self.get_color(),
                self.get_centered(),
                self.get_fixed(),
            ),
            self.get_position().0,
            self.get_position().1,
        );
    }
}
