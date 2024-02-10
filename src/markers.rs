use sdl2::rect::Point;

#[derive(Debug, Clone, Copy)]
pub struct Marker {
    pos: Point,
}
impl Marker {
    pub(crate) fn new(x: i32, y: i32) -> Marker {
        Marker {
            pos: Point::new(x, y),
        }
    }

    pub fn get_pos(&self) -> (usize, usize) {
        // y,x
        (self.pos.y as usize, self.pos.x as usize)
    }
}
