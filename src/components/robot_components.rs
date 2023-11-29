use specs::{Component, VecStorage};

#[derive(Debug)]
pub enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl Component for Facing {
    type Storage = VecStorage<Self>;
}
