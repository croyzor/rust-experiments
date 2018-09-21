extern crate rand;

use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;

#[derive(Debug)]
struct Pos {
    column: usize,
    row:    usize,
}

struct Game {
    board: Vec<Vec<Option<u8>>>,
    score: u32,
    rng:   ThreadRng,
}

// TODO: Make this more generic
fn print_row(row: &Vec<Option<u8>>) {
    let tiles = row.iter().map(|&x| match x {
        Some(i) => i.to_string(),
        None => " ".to_string()
    });
    print!("| ");
    for i in tiles {
        print!("{}", i);
        print!(" | ");
    }
    print!("\n");
}

impl Game {

    fn new(rng: ThreadRng) -> Game {
        Game {
            board: vec![vec![None,None,None,None],
                        vec![None,None,None,None],
                        vec![None,None,None,None],
                        vec![None,None,None,None]],
            score: 0,
            rng: rng,
        }.add_tile()
    }

    fn print(&self) {
        println!("+---+---+---+---+");
        for i in self.board.iter() {
            // TODO: add padding for aligning with double-digit numbers
            print_row(i);
            println!("+---+---+---+---+");
        }
    }

    fn empty_tiles(&self) -> Vec<Pos> {
        let mut x = 0;
        let mut y = 0;
        let mut result = Vec::new();

        for i in self.board.iter() {
            x = 0;
            for j in i.iter() {
                match *j {
                    Some(_) => (),
                    None    => result.push(Pos { column: x,
                                                 row: y }),
                }
                x += 1;
            }
            y += 1;
        }
        result
    }

    fn update(mut self, pos: &Pos, val: u8) -> Self{
        self.board[pos.row][pos.column] = Some(val);
        self
    }

    fn add_tile(mut self) -> Self {
        let options = self.empty_tiles();
        match self.rng.choose(&options) {
            Some(pos) => self.update(pos, 1),
            // TODO: this shouldn't be a panic
            None => panic!("no empty tiles!"),
        }
    }
}

fn main() {
    let mut game = Game::new(thread_rng());
    game.add_tile()
        .add_tile()
        .add_tile()
        .print();
}
