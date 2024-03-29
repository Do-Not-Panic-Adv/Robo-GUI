#[derive(Debug)]
pub struct Position(pub(crate) Point);

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Sprite {
    pub(crate) region: Rect,
    pub(crate) sprite_type: SpriteType,
}

impl Component for Sprite {
    type Storage = VecStorage<Self>;
}
