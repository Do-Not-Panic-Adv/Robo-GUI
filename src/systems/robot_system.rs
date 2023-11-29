use specs::{ReadStorage, System, WriteStorage};

use crate::components::drawable_components::Position;
use crate::components::movement_components::Velocity;

pub(crate) struct RobotSystem;

impl<'a> System<'a> for RobotSystem {
    //there are the resources requires for the execution of the system
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (mut pos, vel): Self::SystemData) {
        //this system will only work with
        //entitis that have both a Position and a Velocity components
        use specs::Join;

        //the join combines multiple component storages, so we access only entities that use both
        //of them

        for (pos, vel) in (&mut pos, &vel).join() {
            println!("ROBOT {:?} {:?}", pos, vel);
            match vel.direction {
                robotics_lib::interface::Direction::Up => pos.0.y -= vel.speed,
                robotics_lib::interface::Direction::Down => pos.0.y -= vel.speed,
                robotics_lib::interface::Direction::Left => pos.0.x -= vel.speed,
                robotics_lib::interface::Direction::Right => pos.0.x += vel.speed,
            }
        }
    }
}
