#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point(pub u32, pub u32, pub u32);

impl Point {
    #[inline]
    pub fn distance_to(&self, other: &Self) -> f32 {
        let dx: f32 = (self.0 as i32 - other.0 as i32) as f32;
        let dy: f32 = (self.1 as i32 - other.1 as i32) as f32;
        let dz: f32 = (self.2 as i32 - other.2 as i32) as f32;

        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}
