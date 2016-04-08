#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector2D {
    pub x: i32,
    pub y: i32
}

impl Vector2D {
    pub fn new(x: i32, y: i32) -> Vector2D {
        Vector2D {
            x: x,
            y: y
        }
    }
}
