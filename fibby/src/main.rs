extern crate rand;
extern crate ggez;
extern crate fibby;

use rand::thread_rng;
use ggez::*;
use ggez::event::{Keycode, Mod};
use ggez::graphics::{DrawMode, Point2, Rect, Color};
use fibby::{Dir,EndGame,Game};

struct State {
    game: Game,
    prev_board: Vec<Vec<Option<u8>>>,
    transition_progress: f32,
    transition_direction: Dir,
    endgame: Option<EndGame>,
}

impl State {
    fn new() -> GameResult<State> {
        Ok(State {
            game: Game::new(thread_rng()),
            prev_board: Vec::new(),
            transition_progress: 0.0,
            transition_direction: Dir::Up,
            endgame: None,
        })
    }
}

fn is_key_valid(keycode: &Keycode) -> bool {
    match keycode {
        Keycode::Up    => true,
        Keycode::Down  => true,
        Keycode::Left  => true,
        Keycode::Right => true,
        _ => false,
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

fn draw_game_over(ctx: &mut Context) -> GameResult<()> {
    graphics::set_color(ctx, Color::new(1.0, 1.0, 1.0, 0.6))?;
    graphics::rectangle(ctx,
                        DrawMode::Fill,
                        Rect {
                            x: 0.0,
                            y: 0.0,
                            w: 500.0,
                            h: 300.0,
                        })?;

    let font = graphics::Font::new(ctx,
                                   "/DejaVuSerif.ttf",
                                   72)?;
    let game_text = graphics::Text::new(ctx, "GAME", &font)?;
    let over_text = graphics::Text::new(ctx, "OVER", &font)?;

    graphics::set_color(ctx, Color::new(1.0, 0.0, 0.0, 1.0))?;
    graphics::draw(ctx,
                   &game_text,
                   Point2::new(100.0,
                               30.0),
                   0.0)?;
    graphics::draw(ctx,
                   &over_text,
                   Point2::new(110.0,
                               135.0),
                   0.0)?;
    Ok(())
}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if timer::check_update_time(ctx, 30) {
            self.transition_progress += 0.03;
            Ok(())
        }
        else {
            timer::yield_now();
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
        if self.endgame.is_some() {
            draw_game_over(ctx)?;
        }
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode,
                      keymod: Mod, repeat: bool) {
        if !is_key_valid(&keycode) {
            return
        }

        let dir = match keycode {
            Keycode::Right => Dir::Right,
            Keycode::Left  => Dir::Left,
            Keycode::Up    => Dir::Up,
            Keycode::Down  => Dir::Down,
            _     => panic!("We shouldn't have got here!"),
        };

        let gg = self.game.clone();
        self.game = gg.shift(&dir);

        self.transition_progress = 0.0;
        self.transition_direction = dir;
        self.endgame = self.game.endgame();
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
