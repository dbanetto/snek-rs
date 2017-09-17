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
            &Direction::North => { pnt.y -= delta },
            &Direction::East => { pnt.x += delta },
            &Direction::South => { pnt.y += delta },
            &Direction::West => { pnt.x -= delta },
        }
    }


    pub fn to_points(start: &Point, path: &VecDeque<Direction>, delta: f32) -> Vec<Point> {

        let mut points = Vec::with_capacity(1 + path.len());
        let mut pos = start.clone();

        points.push(pos);

        for d in path {
            d.oppisite().update_point(&mut pos, delta);
            points.push(pos);
        }

        points
    }
}
