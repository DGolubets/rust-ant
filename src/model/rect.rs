#[derive(Debug)]
pub struct Rect {
    pub x0: i32,
    pub y0: i32,
    pub x1: i32,
    pub y1: i32
}

impl Rect {
    pub fn new(x0: i32, y0: i32, width: u32, height: u32) -> Rect {
        Rect {
            x0: x0,
            y0: y0,
            x1: x0 + width as i32,
            y1: y0 + height as i32
        }
    }

    pub fn get_width(&self) -> u32 {
        (self.x1 - self.x0) as u32
    }

    pub fn get_height(&self) -> u32 {
        (self.y1 - self.y0) as u32
    }
}
