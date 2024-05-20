use crate::{texture_manager::TextureType, MainState, ORD_TEXT, TILE_SIZE};

use super::draw::Drawable;

pub(crate) struct Text {
    text: String,
    position: (i32, i32),
    scale: f32,
    use_global_position: bool, // per implementarlo bisogna cambiare la definizione di FontCharacter
}

impl Text {
    pub fn new(text: String, position: (i32, i32), scale: f32, global: bool) -> Self {
        Self {
            text,
            position,
            scale,
            use_global_position: global,
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
}

impl Drawable for Text {
    fn draw(&self, state: &mut MainState) {
        let mut x = self.get_position().0;
        for c in self.text.chars() {
            MainState::add_drawable(
                &mut state.worlds,
                &state.sprite_table,
                ORD_TEXT,
                TextureType::FontCharater(c, self.scale, self.use_global_position),
                x,
                self.get_position().1,
            );
            x += (TILE_SIZE as f32 * 0.5) as i32;
        }
    }
}
