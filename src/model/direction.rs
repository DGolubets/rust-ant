use super::vector::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Top,
    Bottom
}

impl Direction {
    pub fn to_vector(self) -> Vector2D {
        match self {
            Direction::Left => Vector2D { x: -1, y: 0 },
            Direction::Right => Vector2D { x: 1, y: 0 },
            Direction::Top => Vector2D { x: 0, y: 1 },
            Direction::Bottom => Vector2D { x: 0, y: -1 }
        }
    }

    pub fn from_vector(v: &Vector2D) -> Direction {
        if v.x > 0 { Direction::Right }
        else if v.x < 0 { Direction::Left }
        else if v.y > 0 { Direction::Top }
        else if v.y < 0 { Direction::Bottom }
        else { panic!("Unsupported direction vector: {:?}", v) }
    }
}
