#[derive(Debug)]
pub struct Vector2D<T> {
    pub x: T,
    pub y: T,
}

impl Vector2D<f32> {
    pub fn get(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}
