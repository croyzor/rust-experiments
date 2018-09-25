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
            Game::print_row(i);
            println!("+---+---+---+---+");
        }
    }

    fn empty_tiles(&self) -> Vec<Pos> {
        let mut y = 0;
        let mut result = Vec::new();

        for i in self.board.iter() {
            let mut x = 0;
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

    fn count_tiles(&self) -> u8 {
        let mut total = 0;
        for i in self.board.iter() {
            for j in i.iter() {
                match *j {
                    Some(_) => total += 1,
                    None => (),
                }
            }
        }
        total
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

    fn shift_row_left(row: &mut Vec<Option<u8>>) -> Vec<Option<u8>> {
        let length_orig = row.len();
        row.retain(|a| a.is_some());
        let length_new = row.len();
        for i in 0..(length_orig - length_new) {
            row.push(None);
        }
        row.to_vec()
    }

    fn shift_add_row_left(row: &mut Vec<Option<u8>>) -> Vec<Option<u8>> {
        unimplemented!();
    }
}

fn main() {
    Game::new(thread_rng())
        .add_tile()
        .add_tile()
        .add_tile()
        .print();
}

#[cfg(test)]
mod tests {
    use rand::thread_rng;
    use Game;

    #[test]
    fn initial_board_has_one_tile() {
        assert_eq!(1, Game::new(thread_rng()).count_tiles());
    }

    #[test]
    fn shift_add1() {
        let mut row = vec![Some(1), Some(1), None, None];
        assert_eq!(Game::shift_add_row_left(&mut row), vec![ Some(2), None, None, None ]);
    }
    
    #[test]
    fn shift_add2() {
        let mut row = vec![ Some(1), None, Some(1), None ];
        assert_eq!(Game::shift_add_row_left(&mut row), vec![ Some(2), None, None, None ]);
    }
                         
    #[test]
    fn shift_add3() {
        let mut row = vec![ Some(1), None, None, None ] ;
        assert_eq!(Game::shift_add_row_left(&mut row), vec![ Some(1), None, None, None ]);
    }
    
    #[test]
    fn shift_add4() {
        let mut row = vec![ None, None, Some(1), None ] ;
        assert_eq!(Game::shift_add_row_left(&mut row), vec![ Some(1), None, None, None ]);
    }
    
    #[test]
    fn shift_add5() {
        let mut row = vec![ Some(1), Some(5), None, None ];
        assert_eq!(Game::shift_add_row_left(&mut row), vec![ Some(1), Some(5), None, None ]);
    }
    
    #[test]
    fn shift_add6() {
        let mut row = vec![ Some(5), Some(1), Some(1), None];
        assert_eq!(Game::shift_add_row_left(&mut row), vec![ Some(5), Some(2), None, None ]);
    }
    
    #[test]
    fn shift_add7() {
        let mut row = vec![ Some(1), Some(1), Some(1), Some(1)];
        assert_eq!(Game::shift_add_row_left(&mut row), vec![ Some(2), Some(2), None, None ]);
    }
        
    #[test]
    fn shift_add8() {
        let mut row = vec![ Some(1), Some(1), Some(1), None];
        assert_eq!(Game::shift_add_row_left(&mut row), vec![ Some(2), Some(1), None, None ]);
    }
        
    #[test]
    fn shift1() {
        let mut row = vec! [ Some(1), Some(1), None, None ];
        assert_eq!(Game::shift_row_left(&mut row), vec![ Some(1), Some(1), None, None ]);
    }
                
    #[test]
    fn shift2() {
        let mut row = vec! [ Some(1), None, Some(1), None];
        assert_eq!(Game::shift_row_left(&mut row), vec![ Some(1), Some(1), None, None ]);
    }
                    
    #[test]
    fn shift3() {
        let mut row = vec! [ Some(1), None, None, None ];
        assert_eq!(Game::shift_row_left(&mut row), vec![ Some(1), None, None, None ]);
    }
                    
    #[test]
    fn shift4() {
        let mut row = vec! [ None, None, Some(1), None ];
        assert_eq!(Game::shift_row_left(&mut row), vec![ Some(1), None, None, None ]);
    }
                    
    #[test]
    fn shift5() {
        let mut row = vec! [ Some(1), Some(2), None, None ];
        assert_eq!(Game::shift_row_left(&mut row), vec![ Some(1), Some(2), None, None ]);
    }
}
