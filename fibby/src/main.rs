extern crate rand;

use rand::Rng;

struct Game {
    board: Vec<Vec<Option<u8>>>,
    score: u32,
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
    println!(" |");
}

impl Game {

    fn new() -> Game {
        Game {
            board: vec![vec![None,None,None,None],
                        vec![None,None,None,None],
                        vec![None,None,None,None],
                        vec![None,None,None,None]],
            score: 0
        }
    }

    fn print(&self) {
        println!("+---+---+---+---+");
        for i in self.board.iter() {
            // TODO: add padding for aligning with double-digit numbers
            print_row(i);
            println!("+---+---+---+---+");
        }
    }

    fn count_empty(&self) -> u8 {
        let mut total: u8 = 0;
        for row in self.board.iter() {
            for elem in row.iter() {
                match *elem {
                    Some(_) => total += 1,
                    None    => ()
                };
            }
        }
        return total;
    }
}

fn main() {
    let game = Game::new();
    game.print();
    println!("{}", game.count_empty());
}
