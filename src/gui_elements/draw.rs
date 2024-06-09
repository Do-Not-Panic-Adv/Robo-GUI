use crate::MainState;

pub(crate) trait Drawable {
    fn draw(&self, state: &mut MainState);
    fn get_layer(&self) -> u32;
}
