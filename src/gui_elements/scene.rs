use super::draw::Drawable;

struct Scene<'a> {
    layer: u32,
    elements: Vec<&'a dyn Drawable>,
}

impl<'a> Scene<'a> {
    fn new(layer: u32) -> Self {
        Scene {
            layer,
            elements: Vec::new(),
        }
    }
    fn add_element(&mut self, element: &'a dyn Drawable) {
        self.elements.push(element);
    }

    //TODO: print the real thing
    fn draw(&self) -> String {
        let mut result = String::new();
        for element in &self.elements {
            //result.push_str(&element.draw());
        }
        result
    }
}
