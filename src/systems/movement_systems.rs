use robotics_lib::interface::Direction;
use specs::{Join, ReadExpect, ReadStorage, System, WriteStorage};

use crate::components::drawable_components::Position;
use crate::components::movement_components::Velocity;

pub(crate) struct MoveSystem;

impl<'a> System<'a> for MoveSystem {
    //there are the resources requires for the execution of the system
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (mut pos, vel): Self::SystemData) {
        //this system will only work with
        //entitis that have both a Position and a Velocity components

        //the join combines multiple component storages, so we access only entities that use both
        //of them

        for (pos, vel) in (&mut pos, &vel).join() {
            match &vel.direction {
                Some(d) => match d {
                    robotics_lib::interface::Direction::Up => pos.0.y -= vel.speed,
                    robotics_lib::interface::Direction::Down => pos.0.y += vel.speed,
                    robotics_lib::interface::Direction::Left => pos.0.x -= vel.speed,
                    robotics_lib::interface::Direction::Right => pos.0.x += vel.speed,
                },
                None => return,
            }
        }
    }
}

pub(crate) struct ChangeDirectionSystem;
impl<'a> System<'a> for ChangeDirectionSystem {
    type SystemData = (
        ReadExpect<'a, Option<Direction>>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let dir = &*data.0;
        for vel in (&mut data.1).join() {
            println!("GUIIIII2 {:?}", dir.clone().unwrap());
            vel.direction = dir.clone()
        }
    }
}
