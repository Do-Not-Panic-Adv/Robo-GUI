use std::collections::HashMap;

use sdl2::rect::Point;

#[derive(Debug, Clone)]
pub struct Markers(HashMap<(i32, i32), Marker>);

impl Markers {
    pub(crate) fn new() -> Self {
        Markers(HashMap::new())
    }

    pub(crate) fn toggle(&mut self, coords: (i32, i32)) -> bool {
        // returns false if toggled off, true if toggled on
        //
        //converts to (y,x) used by the robotics lib
        let coords = (coords.1, coords.0);
        if let Some(_) = self.0.get(&coords) {
            self.0.remove(&coords);
            false
        } else {
            self.0.insert(coords, Marker::new(0, 0));
            true
        }
    }
    pub(crate) fn get_all(&self) -> Vec<((i32, i32), Marker)> {
        let mut tmp: Vec<((i32, i32), Marker)> = Vec::new();
        for (coo, mar) in self.0.iter() {
            tmp.push((coo.clone(), mar.clone()));
        }
        tmp
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Marker {
    // doesnt make much sense, keeping it anyway
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
