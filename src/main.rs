extern crate ggez;
extern crate rand;
extern crate recs;

mod ecs;

use std::time::Duration;
use std::collections::VecDeque;

use ecs::*;
use rand::distributions::IndependentSample;

use recs::*;
use ggez::{graphics, timer, Context, GameResult};
use ggez::conf::{self, Conf};
use ggez::event::*;
use ggez::graphics::{Color, Point2, Rect};

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
        let player_pos = Point2::new(50.0, 50.0);
        let _ = ecs.set(player, player_pos);
        let mut tail = VecDeque::with_capacity(10);

        // start the player with a 3 length tail
        tail.push_front(player_pos);
        tail.push_front(player_pos);
        tail.push_front(player_pos);

        let _ = ecs.set(player, tail);
        let _ = ecs.set(player, Direction::East);
        let _ = ecs.set(player, Player);

        MainState {
            player: player,
            ecs: ecs,
            tick: Duration::new(0, 0),
            tick_duration: 250_000_000,
            input: None,
            dot: None,
        }
    }

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
        let y_range = rand::distributions::Range::new(1, (screen.h / 10.0) as u32 - 1);
        let mut rng = rand::thread_rng();

        let x: f32 = x_range.ind_sample(&mut rng) as f32 * 10.0;
        let y: f32 = y_range.ind_sample(&mut rng) as f32 * 10.0;

        let dot_pos = Point2::new(x, y);

        let dot_id = self.ecs.create_entity();
        let _ = self.ecs.set(dot_id, dot_pos);
        let _ = self.ecs.set(dot_id, Dot);

        self.dot = Some(dot_id);
    }

    fn handle_tail(&mut self, keep_tail: bool) {
        let pos = { self.ecs.borrow::<Point2>(self.player).unwrap().clone() };
        let path = self.ecs
            .borrow_mut::<VecDeque<Point2>>(self.player)
            .unwrap();

        let _ = path.pop_back();

        if path.iter().any(|p| p == &pos) {
            println!("{:?} is in {:?}", pos, path.len());
            println!("COLLISION");
        }

        let _ = path.push_front(pos);

        // Ensures that the tail growth only effects player after
        // the tail leaves the dot posision
        if keep_tail {
            let _ = path.push_front(pos);
        }
    }

    fn build_wall(&mut self, ctx: &mut Context) {
        let screen = graphics::get_screen_coordinates(ctx);

        let top_id = self.ecs.create_entity();
        let _ = self.ecs.set(
            top_id,
            Wall {
                size: Rect::new(0.0, 0.0, screen.w, 10.0),
            },
        );

        let bottom_id = self.ecs.create_entity();
        let _ = self.ecs.set(
            bottom_id,
            Wall {
                size: Rect::new(0.0, screen.h - 10.0, screen.w, 10.0),
            },
        );
    }

    fn update_position(&mut self, ctx: &mut Context, dir: &Direction) {
        let screen = graphics::get_screen_coordinates(ctx);

        let mut point = self.ecs.borrow_mut::<Point2>(self.player).unwrap();
        dir.update_point(&mut point, 10.0);

        // handle wrapping around the screen
        if point.x < 0.0 {
            point.x = screen.w - 10.0;
        } else if point.x >= screen.w {
            point.x = 0.0;
        }
        if point.y < 0.0 {
            point.y = screen.h - 10.0;
        } else if point.y >= screen.h {
            point.y = 0.0;
        }
    }

    fn has_hit_wall(&self) -> bool {
        let mut point = self.ecs.borrow::<Point2>(self.player).unwrap().clone();

        // center point to middle of box
        point.x += 5.0;
        point.y += 5.0;

        let filter = component_filter!(Wall);
        let mut wall_ids = vec![];
        let _ = self.ecs.collect_with(&filter, &mut wall_ids);

        wall_ids
            .into_iter()
            .map(|id| self.ecs.borrow::<Wall>(id).unwrap())
            .any(|wall| wall.size.contains(point))
    }

    fn handle_dot(&mut self, ctx: &mut Context) -> bool {
        let mut keep_tail = false;

        if let Some(dot_id) = self.dot {
            {
                let pos = self.ecs.borrow::<Point2>(self.player).unwrap();
                let dot_pos = self.ecs.borrow::<Point2>(dot_id).unwrap();

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

        keep_tail
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = timer::get_delta(ctx);
        self.tick += dt;

        // check if an update tick is in order
        if self.tick <= Duration::new(0, self.tick_duration) {
            return Ok(());
        }

        self.tick = Duration::new(0, 0);

        let direction = self.update_direction();

        // reset buffered input
        self.input = None;

        self.update_position(ctx, &direction);

        let keep_tail = self.handle_dot(ctx);

        self.handle_tail(keep_tail);

        if self.has_hit_wall() {
            println!("WALL COLLISION!!");
        }

        if keep_tail {
            // speed up over time
            self.tick_duration = (self.tick_duration as f64 * 0.98) as u32
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);


        let _ = graphics::set_color(ctx, Color::from((100, 100, 255)));

        let path = self.ecs.borrow::<VecDeque<Point2>>(self.player).unwrap();

        for tail in path {
            let _ = graphics::rectangle(
                ctx,
                graphics::DrawMode::Fill,
                Rect::new(tail.x, tail.y, 10.0, 10.0),
            );
        }

        if let Some(dot_id) = self.dot {
            let _ = graphics::set_color(ctx, Color::from((255, 100, 100)));
            let point = self.ecs.borrow::<Point2>(dot_id).unwrap();

            let _ = graphics::circle(
                ctx,
                graphics::DrawMode::Fill,
                Point2::new(point.x + 5.0, point.y + 5.0),
                5.0,
                5.0,
            );
        }

        let mut wall = vec![];
        let wall_filter = component_filter!(Wall);
        let _ = graphics::set_color(ctx, Color::from((100, 100, 100)));
        self.ecs.collect_with(&wall_filter, &mut wall);
        for id in wall {
            let wall = self.ecs.borrow::<Wall>(id).unwrap();
            let _ = graphics::rectangle(ctx, graphics::DrawMode::Fill, wall.size);
        }

        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        _repeat: bool,
    ) {
        let selected_dir = match keycode {
            Keycode::W => Direction::North,
            Keycode::D => Direction::East,
            Keycode::S => Direction::South,
            Keycode::A => Direction::West,
            _ => return,
        };

        let back = self.ecs.borrow::<Direction>(self.player).unwrap();

        // prevent player from selecting backwards onto themselves
        if selected_dir != back.oppisite() {
            self.input = Some(selected_dir);
        }
    }
}

fn main() {
    let conf = Conf {
        window_mode: conf::WindowMode {
            width: 400,
            height: 400,
            ..conf::WindowMode::default()
        },
        window_setup: conf::WindowSetup {
            title: "snek".to_owned(),
            ..conf::WindowSetup::default()
        },
        backend: conf::Backend::default(),
    };

    let mut context = Context::load_from_conf("snek", "snek", conf).unwrap();
    let mut state = MainState::new();
    state.build_wall(&mut context);

    if let Err(err) = run(&mut context, &mut state) {
        println!("{:?}", err);
    }
}
