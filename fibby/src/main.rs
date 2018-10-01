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
        for (i, row) in self.game.get_board().iter().enumerate() {
            for (j, elem) in row.iter().enumerate() {
                graphics::rectangle(ctx,
                                    DrawMode::Fill,
                                    Rect {
                                        x: 100.0 * j as f32,
                                        y: 100.0 * i as f32,
                                        w: 100.0,
                                        h: 100.0,
                                    })?;
                /*
                let font = graphics::Font::new(ctx,
                                               "/DejaVuSerif.ttf",
                                               48)?;
                 */
                let font = graphics::Font::default_font()?;
                let text = graphics::Text::new(ctx, "a", &font)?;
                graphics::draw(ctx,
                               &text,
                               Point2::new(100.0 * j as f32,
                                           100.0 * i as f32),
                               0.0)?;
            }
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
