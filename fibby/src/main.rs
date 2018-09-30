extern crate rand;
extern crate ggez;
extern crate fibby;

use rand::thread_rng;
use ggez::*;
use ggez::graphics::{DrawMode, Point2, Rect};
use fibby::{Dir,Game};

struct State {
    game: Game,
}

impl State {
    fn new() -> GameResult<State> {
        Ok(State { game: Game::new(thread_rng()) })
    }
}

impl event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        let mut x = 0.0;
        for i in self.game.get_board().iter() {
            let mut y = 0.0;
            for _ in i {
                graphics::rectangle(ctx,
                                    DrawMode::Fill,
                                    Rect {
                                        x: x,
                                        y: y,
                                        w: 50.0,
                                        h: 50.0,
                                    })?;
                x += 50.0;
            }
            y += 50.0;
        }
        graphics::present(ctx);
        Ok(())
    }
}

fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("2048", "ggez", c).unwrap();
    let state = &mut State::new().unwrap();
    event::run(ctx, state).unwrap();
}
