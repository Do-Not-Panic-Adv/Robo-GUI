use crate::MainState;

use super::draw::Drawable;

pub(crate) struct Scene {
    name: String,
    layer: u32,
    elements: Vec<Box<dyn Drawable>>,
}

impl Scene {
    pub(crate) fn new(name: String, layer: u32) -> Self {
        Scene {
            layer,
            elements: Vec::new(),
            name,
        }
    }
    pub(crate) fn add_element(&mut self, mut element: Box<dyn Drawable>) {
        element.set_parent((self.name.clone(), self.layer));
        self.elements.push(element);
    }
    pub(crate) fn get_name(&self) -> String {
        self.name.clone()
    }
    pub(crate) fn get_layer(&self) -> u32 {
        self.layer
    }

    //TODO: print the real thing
    pub(crate) fn draw(&self, state: &mut MainState) {
        for element in &self.elements {
            element.draw(state)
        }
    }
}

pub(crate) enum MainScenes {
    Inventory,
    Markers,
}
