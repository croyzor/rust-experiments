extern crate rand;
extern crate ggez;
extern crate fibby;

use rand::thread_rng;
use ggez::*;
use ggez::event::{Keycode, Mod};
use ggez::graphics::{DrawMode, Point2, Rect, Color};
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
                graphics::set_color(ctx, Color::new(1.0, 1.0, 1.0, 1.0))?;
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

                graphics::set_color(ctx, match elem {
                    Some(1) => Color::new(1.0, 0.0, 0.0, 1.0),
                    Some(2) => Color::new(0.0, 1.0, 0.0, 1.0),
                    Some(4) => Color::new(0.0, 0.0, 1.0, 1.0),
                    Some(_) => Color::new(1.0, 1.0, 1.0, 1.0),
                    None => Color::new(1.0,1.0,1.0,1.0),
                })?;
                graphics::draw(ctx,
                               &text,
                               Point2::new(100.0 * j as f32 + 32.0,
                                           100.0 * i as f32 + 17.0),
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
    let mut c = conf::Conf::new();
    c.window_setup.title = "144".to_string();
    c.window_mode.width = 400;
    c.window_mode.height = 400;
    let ctx = &mut Context::load_from_conf("144", "croyzor", c).unwrap();
    let state = &mut State::new().unwrap();
    event::run(ctx, state).unwrap();
}
