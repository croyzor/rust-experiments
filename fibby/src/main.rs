extern crate rand;
extern crate ggez;
extern crate fibby;

use rand::thread_rng;
use ggez::*;
use ggez::event::{Keycode, Mod};
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
                                    DrawMode::Line(8.0),
                                    Rect {
                                        x: 100.0 * j as f32,
                                        y: 100.0 * i as f32,
                                        w: 100.0,
                                        h: 100.0,
                                    })?;
                let font = graphics::Font::new(ctx,
                                               "/DejaVuSerif.ttf",
                                               48)?;
                let num = match elem {
                    None => "".to_string(),
                    Some(i) => i.to_string(),
                };
                let text = graphics::Text::new(ctx, &num, &font)?;
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

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode,
                      keymod: Mod, repeat: bool) {
        let gg = self.game.clone();
        self.game = match keycode {
            Keycode::Right => gg.shift(Dir::Right),
            Keycode::Left  => gg.shift(Dir::Left),
            Keycode::Up    => gg.shift(Dir::Up),
            Keycode::Down  => gg.shift(Dir::Down),
            _     => gg,
        };
    }
}

fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("2048", "ggez", c).unwrap();
    let state = &mut State::new().unwrap();
    event::run(ctx, state).unwrap();
}
