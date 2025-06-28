use crate::toolbox::Coordinates;

#[derive(Debug)]
pub struct SecurityRobot {
    starting_position: Coordinates,
    speed: Coordinates,
}

impl SecurityRobot {
    pub fn new(x0: i32, y0: i32, vx: i32, vy: i32) -> Self {
        Self {
            starting_position: Coordinates { x: x0, y: y0 },
            speed: Coordinates { x: vx, y: vy },
        }
    }

    pub fn progress_for(&self, seconds: i32) -> Coordinates {
        self.starting_position + (self.speed * seconds)
    }
}
