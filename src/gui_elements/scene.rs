use crate::MainState;

use super::draw::Drawable;
use uuid::Uuid;

pub(crate) struct Scene {
    name: String,
    id: Uuid,
    layer: u32,
    elements: Vec<Box<dyn Drawable>>,
}

impl Scene {
    pub(crate) fn new(name: String, layer: u32) -> Self {
        Scene {
            layer,
            elements: Vec::new(),
            name,
            id: Uuid::new_v4(),
        }
    }
    pub(crate) fn add_element(&mut self, element: Box<dyn Drawable>) {
        self.elements.push(element);
    }
    pub(crate) fn get_name(&self) -> String {
        self.name.clone()
    }
    pub(crate) fn get_id(&self) -> Uuid {
        self.id
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
