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

pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alfa: f32,
}

impl Color {
    pub fn get(&self) -> (f32, f32, f32, f32) {
        (self.red, self.green, self.blue, self.alfa)
    }
}

pub struct Frames {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Frames {
    pub fn get(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }
}

pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn get(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}
