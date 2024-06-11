use crate::{renderer::Layer, texture_manager::TextureType, MainState, TILE_SIZE};

use super::draw::Drawable;

#[derive(Clone)]
pub(crate) struct Text {
    text: String,
    position: (i32, i32),
    scale: f32,
    fixed: bool,
    layer: u32,
    parent: Option<(String, u32)>,
}

impl Text {
    pub fn new(text: String, position: (i32, i32), scale: f32, fixed: bool, layer: u32) -> Self {
        Self {
            text,
            position,
            scale,
            fixed,
            layer,
            parent: None,
        }
    }
    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
    pub fn set_position(&mut self, position: (i32, i32)) {
        self.position = position;
    }
    pub fn get_position(&self) -> (i32, i32) {
        self.position
    }
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }
    pub fn get_text(&self) -> String {
        self.text.clone()
    }
    pub fn get_parent(&self) -> Option<(String, u32)> {
        self.parent.clone()
    }
}

impl Drawable for Text {
    fn draw(&self, state: &mut MainState) {
        let mut x = self.get_position().0;
        for c in self.text.chars() {
            state.scenes.push((
                self.get_parent().unwrap().0.clone(),
                self.get_parent().unwrap().1,
                self.get_layer(),
            ));

            MainState::add_ui_element(
                &mut state.ui_elements,
                &state.sprite_table,
                Layer::Ui(
                    self.get_parent().unwrap().0,
                    self.get_parent().unwrap().1,
                    self.get_layer(),
                ),
                TextureType::FontCharater(c, self.scale, self.fixed),
                x,
                self.get_position().1,
            );
            x += (TILE_SIZE as f32 * 0.3 + (TILE_SIZE as f32 * 0.3 * self.scale - 1.0)) as i32;
        }
    }
    fn get_layer(&self) -> u32 {
        self.layer
    }
    fn set_parent(&mut self, _parent: (String, u32)) {
        self.parent = Option::Some(_parent)
    }
}
