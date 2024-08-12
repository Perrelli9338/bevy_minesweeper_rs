use bevy::prelude::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Bounds2 {
    pub position: Vec2,
    pub size: Vec2
}

impl Bounds2 {
    pub fn in_bounds(&self, coordinates: Vec2) -> bool {
        let bottom_left = self.position;
        let top_right = self.position + self.size;
        let x = coordinates.x;
        let y = coordinates.y;
        return (x >= bottom_left.x && x <= top_right.x) && (y >= bottom_left.y && y <= top_right.y);
    }
}