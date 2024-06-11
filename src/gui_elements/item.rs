use crate::{renderer::Layer, texture_manager::TextureType, MainState};

use super::{draw::Drawable, scene::Scene};

pub(crate) struct Item {
    position: (i32, i32),
    scale: f32,
    fixed: bool,
    class: TextureType,
    layer: u32,
    parent: Option<(String, u32)>,
}

impl Item {
    pub fn new(
        position: (i32, i32),
        scale: f32,
        fixed: bool,
        class: TextureType,
        layer: u32,
    ) -> Self {
        Self {
            position,
            scale,
            fixed,
            class,
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
        self.fixed
    }
    pub fn get_parent(&self) -> Option<(String, u32)> {
        self.parent.clone()
    }
}

impl Drawable for Item {
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
                self.get_parent().unwrap().0,
                self.get_parent().unwrap().1,
                self.get_layer(),
            ),
            TextureType::Item(
                Box::new(self.get_class().clone()),
                self.get_scale(),
                self.get_fixed(),
            ),
            self.get_position().0,
            self.get_position().1,
        );
    }
    fn get_layer(&self) -> u32 {
        self.layer
    }
    fn set_parent(&mut self, parent: (String, u32)) {
        self.parent = Some(parent);
    }
}
