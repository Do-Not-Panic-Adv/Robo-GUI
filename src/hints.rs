use sdl2::rect::Point;

#[derive(Debug, Clone, Copy)]
pub struct Hint {
    pos: Point,
}
impl Hint {
    pub(crate) fn new(x: i32, y: i32) -> Hint {
        Hint {
            pos: Point::new(x, y),
        }
    }

    pub fn get_pos(&self) -> (usize, usize) {
        // y,x
        (self.pos.y as usize, self.pos.x as usize)
    }
}
