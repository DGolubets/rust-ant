use super::color::*;

pub struct Map {
    width: usize,
    height: usize,
    cells: Vec<MapCell>
}

#[derive(Clone)]
pub struct MapCell {
    pub color: Color
}

impl Default for MapCell {
    fn default() -> MapCell {
        MapCell { color: Color::Gray }
    }
}

impl Map {
    pub fn new(width: usize, height: usize) -> Map {
        Map {
            width: width,
            height: height,
            cells: vec![Default::default(); width * height]
        }
    }

    pub fn get_color(&self, x: u32, y: u32) -> Color {
        //self.cell(x, y).map_or(Color::Green, |c| c.color)
        self.cell(x, y).unwrap_or(&Default::default()).color
    }

    pub fn set_color(&mut self, x: u32, y: u32, color: Color) {
        if let Some(cell) = self.cell_mut(x, y) {
            cell.color = color
        }
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    fn cell(&self, x: u32, y: u32) -> Option<&MapCell> {
        let index = self.index(x, y);
        if index < self.cells.len() {
            Some(&self.cells[index])
        }
        else {
            None
        }
    }

    fn cell_mut(&mut self, x: u32, y: u32) -> Option<&mut MapCell> {
        let index = self.index(x, y);
        if index < self.cells.len() {
            Some(&mut self.cells[index])
        }
        else {
            None
        }
    }

    fn index(&self, x: u32, y: u32) -> usize {
        y as usize * self.width + x as usize
    }
}

#[cfg(test)]
mod tests {
    use model::*;

    #[test]
    fn default_color_is_white() {
        let map = Map::new(10, 10);
        assert_eq!(Color::White, map.get_color(1, 2));
    }

    #[test]
    fn change_color() {
        let mut map = Map::new(10, 10);
        map.set_color(1, 2, Color::Red);
        assert_eq!(Color::Red, map.get_color(1, 2));
    }

    #[test]
    fn return_default_color_when_out_of_bounds() {
        let map = Map::new(10, 10);
        assert_eq!(Color::White, map.get_color(100, 2));
    }

    #[test]
    fn ignore_set_color_when_out_of_bounds() {
        let mut map = Map::new(10, 10);
        map.set_color(100, 2, Color::Red);
        assert_eq!(Color::White, map.get_color(100, 2));
    }
}
