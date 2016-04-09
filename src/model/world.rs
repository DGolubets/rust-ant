use std::cmp::*;
use super::*;

const DEFAULT_VIEW_SIZE: u32 = 10;

pub struct World {
    time: f64,
    map: Map,
    view: Rect,
    ant: Ant,
    move_number: u32,
    moves_per_second: u32
}

impl World {
    pub fn new(size: u32, mps: u32) -> World {
        let map = Map::new(size as usize, size as usize);
        let ant = Ant::new((map.get_width() / 2) as i32, (map.get_height() / 2) as i32, Direction::Right);
        let view = Rect::new(
            ant.get_position().x - DEFAULT_VIEW_SIZE as i32 / 2,
            ant.get_position().y - DEFAULT_VIEW_SIZE as i32 / 2,
            DEFAULT_VIEW_SIZE,
            DEFAULT_VIEW_SIZE
        );

        World {
            time: 0.0,
            map: map,
            view: view,
            ant: ant,
            move_number: 0,
            moves_per_second: mps
        }
    }

    pub fn get_ant(&self) -> &Ant {
        &self.ant
    }

    pub fn get_map(&self) -> &Map {
        &self.map
    }

    pub fn get_view(&self) -> &Rect {
        &self.view
    }

    pub fn tick(&mut self, dt: f64) {
        self.time += dt;
        let target_moves_number = (self.time * self.moves_per_second as f64) as u32;
        let diff_moves_number = target_moves_number - self.move_number;

        for _ in 0..diff_moves_number {
            self.move_ant();
            self.move_number += 1;
            println!("{}", self.move_number);
            self.extend_view();
        }
    }

    fn extend_view(&mut self) {
        let pos = self.get_ant().get_position();
        self.view.x0 = max(min(self.view.x0, pos.x), 0);
        self.view.y0 = max(min(self.view.y0, pos.y), 0);
        self.view.x1 = min(max(self.view.x1, pos.x), self.map.get_width() as i32);
        self.view.y1 = min(max(self.view.y1, pos.y), self.map.get_height() as i32);
    }

    fn move_ant(&mut self){
        let pos = self.ant.get_position();
        let color = self.map.get_color(pos.x as u32, pos.y as u32);
        match color {
            Color::White | Color:: Gray => {
                self.ant.turn_left();
                self.map.set_color(pos.x as u32, pos.y as u32, Color::Red);
            },
            Color::Red => {
                self.ant.turn_right();
                self.map.set_color(pos.x as u32, pos.y as u32, Color::White);
            },
            Color::Green => {
                //self.ant.turn_right();
                //self.map.set_color(pos.x as u32, pos.y as u32, Color::White);
            }
        }
        self.ant.move_forward();
        //println!("{:?}", self.ant);
    }
}
