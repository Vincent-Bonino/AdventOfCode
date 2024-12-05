use crate::toolbox::Direction;

#[derive(Copy, Clone, Debug)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

impl Coordinates {
    pub fn step(&self, direction: &Direction) -> Self {
        let (delta_x, delta_y): (i32, i32) = direction.get_delta();
        Coordinates {
            x: self.x + delta_x,
            y: self.y + delta_y,
        }
    }
}
