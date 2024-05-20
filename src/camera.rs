use sdl2::rect::Point;

#[derive(Debug)]
pub struct Camera {
    pub screen_offset: (i32, i32),
    pub(crate) chase_robot: bool,
    pub(crate) zoom_level: i32,
    pub(crate) robot_position: Point,
}
