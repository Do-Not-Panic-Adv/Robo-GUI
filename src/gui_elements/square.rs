use sdl2::pixels::Color;

use crate::{renderer::Layer, texture_manager::TextureType, MainState};

use super::{draw::Drawable, scene::Scene};

pub(crate) struct Square {
    position: (i32, i32),
    size: (u32, u32),
    fixed: bool,
    centered: bool,
    color: Color,
    layer: u32,
    parent: Option<(String, u32)>,
}

impl Square {
    pub fn new(
        position: (i32, i32),
        size: (u32, u32),
        fixed: bool,
        centered: bool,
        color: Color,
        layer: u32,
    ) -> Self {
        Self {
            position,
            size,
            fixed,
            centered,
            color,
            layer,
            parent: None,
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
    fn get_parent(&self) -> Option<(String, u32)> {
        self.parent.clone()
    }
}

impl Drawable for Square {
    fn draw(&self, state: &mut MainState) {
        state.scenes.push((
            self.get_parent().unwrap().0.clone(),
            self.get_parent().unwrap().1,
            self.get_layer(),
        ));

        MainState::add_ui_element(
            &mut state.ui_elements,
            &state.sprite_table,
            Layer::Ui(
                self.get_parent().unwrap().0.clone(),
                self.get_parent().unwrap().1,
                self.get_layer(),
            ),
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

    fn get_layer(&self) -> u32 {
        self.layer
    }
    fn set_parent(&mut self, _parent: (String, u32)) {
        self.parent = Some(_parent);
    }
}
