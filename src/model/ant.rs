use super::vector::*;
use super::direction::*;

#[derive(Debug)]
pub struct Ant {
    position: Vector2D,
    direction: Vector2D
}

impl Ant {
    pub fn new(x0: i32, y0: i32, d: Direction) -> Ant {
        Ant {
            position: Vector2D::new(x0, y0),
            direction: d.to_vector()
        }
    }

    pub fn facing(&self) -> Direction {
        Direction::from_vector(&self.direction)
    }

    pub fn get_position(&self) -> Vector2D {
        self.position
    }

    pub fn turn_left(&mut self) {
        let x = self.direction.x;
        let y = self.direction.y;
        self.direction.x = -y;
        self.direction.y = x;
    }

    pub fn turn_right(&mut self) {
        let x = self.direction.x;
        let y = self.direction.y;
        self.direction.x = y;
        self.direction.y = -x;
    }

    pub fn move_forward(&mut self) {
        self.position.x += self.direction.x;
        self.position.y += self.direction.y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::direction::*;
    use super::super::vector::*;

    #[test]
    fn face_left() {
        let mut ant = Ant::new(0, 0, Direction::Top);

        ant.turn_left();
        assert_eq!(Direction::Left, ant.facing())
    }

    #[test]
    fn face_right() {
        let mut ant = Ant::new(0, 0, Direction::Top);

        ant.turn_right();
        assert_eq!(Direction::Right, ant.facing())
    }

    #[test]
    fn face_top() {
        let mut ant = Ant::new(0, 0, Direction::Right);

        ant.turn_left();
        assert_eq!(Direction::Top, ant.facing())
    }

    #[test]
    fn face_bottom() {
        let mut ant = Ant::new(0, 0, Direction::Left);

        ant.turn_left();
        assert_eq!(Direction::Bottom, ant.facing())
    }

    #[test]
    fn move_forward() {
        let mut ant = Ant::new(0, 0, Direction::Right);

        ant.move_forward();
        assert_eq!(Vector2D { x: 1, y: 0 }, ant.position)
    }
}
