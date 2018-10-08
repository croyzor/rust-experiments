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

// Helper function for rendering a tile
fn draw_tile(ctx: &mut Context, data: &Option<u8>, i: usize, j: usize) -> GameResult<()> {
    let fontsize = match data {
        Some(144) => 36,
        _ => 48,
    };

    let font = graphics::Font::new(ctx,
                                   "/DejaVuSerif.ttf",
                                   fontsize)?;
    let num = match data {
        None => "".to_string(),
        Some(i) => i.to_string(),
    };
    let text = graphics::Text::new(ctx, &num, &font)?;

    graphics::set_color(ctx, match data {
        Some(1) => Color::from_rgb(78, 205, 196),
        Some(2) => Color::from_rgb(209, 242, 165),
        Some(3) => Color::from_rgb(249, 150, 180),
        Some(5) => Color::from_rgb(199, 244, 100),
        Some(8) => Color::from_rgb(255, 196, 140),
        Some(13) => Color::from_rgb(255, 159, 128),
        Some(21) => Color::from_rgb(235, 120, 108),
        Some(34) => Color::from_rgb(245, 105, 145),
        Some(_) => Color::from_rgb(255, 107, 107),
        None => Color::new(1.0,1.0,1.0,1.0),
    })?;

    let x_offset = match num.len() {
        1 => 32.0,
        2 => 12.0,
        3 => 8.0,
        _ => 0.0,
    };

    let y_offset = match num.len() {
        3 => 23.0,
        _ => 17.0,
    };

    graphics::draw(ctx,
                   &text,
                   Point2::new(100.0 * j as f32 + x_offset,
                               100.0 * i as f32 + y_offset),
                   0.0)?;
    Ok(())
}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !timer::check_update_time(ctx, 30) {
            timer::yield_now();
            Ok(())
        }
        else {
            Ok(())
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_background_color(ctx,
                                       Color::from_rgb(85, 98, 112));
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
                draw_tile(ctx, &elem, i, j)?;
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
    c.window_mode.width = 500;
    c.window_mode.height = 300;
    c.window_mode.vsync = true;
    let ctx = &mut Context::load_from_conf("144", "croyzor", c).unwrap();
    let state = &mut State::new().unwrap();
    event::run(ctx, state).unwrap();
}
