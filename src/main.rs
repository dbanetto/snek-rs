extern crate ggez;
extern crate recs;
extern crate rand;

mod ecs;

use std::time::Duration;
use std::collections::VecDeque;

use ecs::*;
use rand::distributions::IndependentSample;

use recs::*;
use ggez::conf::Conf;
use ggez::Context;
use ggez::event::*;
use ggez::GameResult;
use ggez::graphics;
use ggez::graphics::{Point, Color, Rect};

struct MainState {
    player: EntityId,
    ecs: Ecs,
    tick: Duration,
    tick_duration: u32,
    input: Option<Direction>,
    dot: Option<EntityId>,
}

impl MainState {
    fn new() -> Self {
        let mut ecs = Ecs::new();

        let player = ecs.create_entity();
        let player_pos = Point::new(50.0, 50.0);
        let _ = ecs.set(player, player_pos);
        let mut tail = VecDeque::with_capacity(10);
        tail.push_front(player_pos);
        tail.push_front(player_pos);
        tail.push_front(player_pos);
        let _ = ecs.set(player, tail);
        let _ = ecs.set(player, Direction::East);
        let _ = ecs.set(player, Object::Player);

        MainState {
            player: player,
            ecs: ecs,
            tick: Duration::new(0, 0),
            tick_duration: 250_000_000,
            input: None,
            dot: None,
        }
    }
}

impl MainState {
    fn update_direction(&mut self) -> Direction {
        match &self.input {
            &Some(ref dir) => {
                let direction = self.ecs.borrow_mut::<Direction>(self.player).unwrap();
                let dir = dir.clone();
                if direction.oppisite() != dir {
                    *direction = dir;
                }
                direction.clone()
            }
            &None => self.ecs.borrow::<Direction>(self.player).unwrap().clone(),
        }
    }

    fn create_dot(&mut self, ctx: &mut Context) {
        let screen = graphics::get_screen_coordinates(ctx);
        let x_range = rand::distributions::Range::new(1, (screen.w / 10.0) as u32 - 1);
        let y_range = rand::distributions::Range::new(1, (-screen.h / 10.0) as u32 - 1);
        let mut rng = rand::thread_rng();

        let x: f32 = x_range.ind_sample(&mut rng) as f32 * 10.0;
        let y: f32 = y_range.ind_sample(&mut rng) as f32 * 10.0;

        let dot_pos = Point::new(x, y);

        let dot_id = self.ecs.create_entity();
        let _ = self.ecs.set(dot_id, dot_pos);

        self.dot = Some(dot_id);
    }

    fn handle_tail(&mut self, keep_tail: bool) {
        let pos = {
            self.ecs.borrow::<Point>(self.player).unwrap().clone()
        };
        let path = self.ecs.borrow_mut::<VecDeque<Point>>(self.player).unwrap();

        let _ = path.pop_back();

        if path.iter().any(|p| p == &pos) {
            println!("{:?} is in {:?}", pos, path);
            println!("COLLISION");
        }

        let _ = path.push_front(pos);

        // Ensures that the tail growth only effects player after
        // the tail leaves the dot posision
        if keep_tail {
            let _ = path.push_front(pos);
        }
    }
}

impl EventHandler for MainState {

    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()> {

        self.tick += dt;

        // check if an update tick is in order
        if self.tick <= Duration::new(0, self.tick_duration) {
            return Ok(());
        }

        self.tick = Duration::new(0, 0);

        let direction = self.update_direction();

        // reset buffered input
        self.input = None;

        {
            let mut point = self.ecs.borrow_mut::<Point>(self.player).unwrap();
            direction.update_point(&mut point, 10.0);
        }

        let mut keep_tail = false;

        if let Some(dot_id) = self.dot {
            {
                let pos = self.ecs.borrow::<Point>(self.player).unwrap();
                let dot_pos = self.ecs.borrow::<Point>(dot_id).unwrap();

                if dot_pos == pos {
                    keep_tail = true;
                }
            }

            if keep_tail {
                let _ = self.ecs.destroy_entity(dot_id);
                self.dot = None;
            }
        } else {
            self.create_dot(ctx);
        }


        self.handle_tail(keep_tail);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        let _ = graphics::set_color(ctx, Color::from((100, 100, 255)));
        let path = self.ecs.borrow::<VecDeque<Point>>(self.player).unwrap();

        for tail in path {
            let _ = graphics::rectangle(
                ctx,
                graphics::DrawMode::Fill,
                Rect::new(tail.x - 5.0, tail.y - 5.0, 10.0, 10.0),
            );
        }

        if let Some(dot_id) = self.dot {
            let _ = graphics::set_color(ctx, Color::from((255, 100, 100)));
            let point = self.ecs.borrow::<Point>(dot_id).unwrap();

            let _ = graphics::circle(
                ctx,
                graphics::DrawMode::Fill,
                Point::new(point.x - 5.0, point.y - 5.0),
                5.0,
                5,
            );
        }

        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        self.input = match keycode {
            Keycode::W => Some(Direction::North),
            Keycode::D => Some(Direction::East),
            Keycode::S => Some(Direction::South),
            Keycode::A => Some(Direction::West),
            _ => return,
        };
    }
}

fn main() {
    let conf = Conf::new();

    let mut context = Context::load_from_conf("snek", "snek", conf).unwrap();
    let mut state = MainState::new();

    if let Err(err) = run(&mut context, &mut state) {
        println!("{:?}", err);
    }
}
