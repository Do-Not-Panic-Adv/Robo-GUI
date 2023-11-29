use robotics_lib::interface::Direction;
use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Velocity {
    pub(crate) speed: i32,
    pub(crate) direction: Direction,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}
