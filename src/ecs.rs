use std::collections::VecDeque;
use ggez::graphics::Point;

#[derive(Clone, PartialEq, Debug)]
pub enum Object {
    Player,
    Wall,
    Dot,
    Fruit,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn oppisite(&self) -> Self {
        match self {
            &Direction::North => Direction::South,
            &Direction::East => Direction::West,
            &Direction::South => Direction::North,
            &Direction::West => Direction::East,
        }
    }

    pub fn update_point(&self, pnt: &mut Point, delta: f32) {
        match self {
            &Direction::North => pnt.y -= delta,
            &Direction::East => pnt.x += delta,
            &Direction::South => pnt.y += delta,
            &Direction::West => pnt.x -= delta,
        }
    }

    pub fn move_by(&self, start: &Point, delta: f32) -> Point {
        match self {
            &Direction::North => Point::new(start.x, start.y - delta),
            &Direction::East => Point::new(start.x + delta, start.y),
            &Direction::South => Point::new(start.x, start.y + delta),
            &Direction::West => Point::new(start.x - delta, start.y),
        }
    }
}
