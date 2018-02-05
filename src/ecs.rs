use ggez::graphics::{Point2, Rect};

#[derive(Clone, PartialEq, Debug)]
pub struct Player;

#[derive(Clone, PartialEq, Debug)]
pub struct Wall {
    pub size: Rect,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Dot;

#[derive(Clone, PartialEq, Debug)]
pub struct Fruit;

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

    pub fn update_point(&self, pnt: &mut Point2, delta: f32) {
        match self {
            &Direction::North => pnt.y -= delta,
            &Direction::East => pnt.x += delta,
            &Direction::South => pnt.y += delta,
            &Direction::West => pnt.x -= delta,
        }
    }

    pub fn move_by(&self, start: &Point2, delta: f32) -> Point2 {
        match self {
            &Direction::North => Point2::new(start.x, start.y - delta),
            &Direction::East => Point2::new(start.x + delta, start.y),
            &Direction::South => Point2::new(start.x, start.y + delta),
            &Direction::West => Point2::new(start.x - delta, start.y),
        }
    }
}
