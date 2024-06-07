use crate::{texture_manager::TextureType, MainState, ORD_TEXT};

use super::draw::Drawable;

pub(crate) struct Item {
    position: (i32, i32),
    scale: f32,
    use_global_position: bool,
    class: TextureType,
}

impl Item {
    pub fn new(
        position: (i32, i32),
        scale: f32,
        use_global_position: bool,
        class: TextureType,
    ) -> Self {
        Self {
            position,
            scale,
            use_global_position,
            class,
        }
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
    pub fn get_scale(&self) -> f32 {
        self.scale
    }
    pub fn get_class(&self) -> TextureType {
        self.class.clone()
    }
    pub fn get_fixed(&self) -> bool {
        self.use_global_position
    }
}

impl Drawable for Item {
    fn draw(&self, state: &mut MainState) {
        println!("added item to drawables");
        MainState::add_drawable(
            &mut state.worlds,
            &state.sprite_table,
            ORD_TEXT,
            TextureType::Item(
                Box::new(self.get_class().clone()),
                self.get_scale(),
                self.get_fixed(),
            ),
            self.get_position().0,
            self.get_position().1,
        );
    }
}
