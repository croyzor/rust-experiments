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

enum Dir {
    Left,
    Right,
    Up,
    Down,
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
        let mut shifted_row = Game::shift_row_left(row);
        if let None = shifted_row[0] {
            return shifted_row;
        }
        else if shifted_row[0] == shifted_row[1] {
            // TODO: there's definitely a better way to do this
            shifted_row[0] = Some(shifted_row[0].unwrap() +
                                  shifted_row[1].unwrap());
            shifted_row.remove(1);
            shifted_row.push(None);
        }
        if shifted_row.len() >= 3 {
            // Call this function on the rest of the row
            let mut rest = shifted_row.split_off(1);
            shifted_row.append(&mut Game::shift_add_row_left(&mut rest));
        }
        shifted_row
    }

    // TODO: make this generic
    fn transpose(mat: &Vec<Vec<Option<u8>>>) -> Vec<Vec<Option<u8>>> {
        let mut result = Vec::new();
        for i in 0..mat[0].len() {
            result.push(mat.iter().map(|a| a[i]).collect());
        }
        result
    }

    fn shift_left(mut self) -> Self {
        // Apply shift_add_row_left to each row
        self.board =
            self.board
            .iter_mut()
            .map(|mut row| Game::shift_add_row_left(&mut row))
            .collect();
        self
    }

    fn shift_right(mut self) -> Self {
        // Reverse each row, call shift_add_row_left, reverse
        self.board =
            self.board
            .iter_mut()
            .map(|mut row| {
                row.reverse();
                let mut result = Game::shift_add_row_left(&mut row);
                result.reverse();
                result
            })
            .collect();
        self
    }

    // Move board tiles in a given direction
    fn shift(mut self, dir: Dir) -> Self {
        match dir {
            Dir::Left  => self.shift_left(),
            Dir::Right => self.shift_right(),
            _ => unimplemented!(),
        }
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
    use Dir;

    #[test]
    fn initial_board_has_one_tile() {
        assert_eq!(1, Game::new(thread_rng()).count_tiles());
    }

    #[test]
    fn shift_left() {
        let mut game = Game::new(thread_rng());
        game.board = vec!(vec![Some(1), Some(1), None, None],
                          vec![Some(1), None, Some(1), None ],
                          vec![ Some(1), None, None, None ],
                          vec![ None, None, Some(1), None ],
                          vec![ Some(1), Some(5), None, None ],
                          vec![ Some(5), Some(1), Some(1), None],
                          vec![ Some(1), Some(1), Some(1), Some(1)],
                          vec![ Some(1), Some(1), Some(1), None]);

        let expected = vec!(vec![ Some(2), None, None, None ],
                            vec![ Some(2), None, None, None ],
                            vec![ Some(1), None, None, None ],
                            vec![ Some(1), None, None, None ],
                            vec![ Some(1), Some(5), None, None ],
                            vec![ Some(5), Some(2), None, None ],
                            vec![ Some(2), Some(2), None, None ],
                            vec![ Some(2), Some(1), None, None ]);
        assert_eq!(expected, game.shift(Dir::Left).board);
    }
        
    #[test]
    fn shift1() {
        let mut row = vec! [ Some(1), Some(1), None, None ];
        assert_eq!(Game::shift_row_left(&mut row),
                   vec![ Some(1), Some(1), None, None ]);
    }
                
    #[test]
    fn shift2() {
        let mut row = vec! [ Some(1), None, Some(1), None];
        assert_eq!(Game::shift_row_left(&mut row),
                   vec![ Some(1), Some(1), None, None ]);
    }
                    
    #[test]
    fn shift3() {
        let mut row = vec! [ Some(1), None, None, None ];
        assert_eq!(Game::shift_row_left(&mut row),
                   vec![ Some(1), None, None, None ]);
    }
                    
    #[test]
    fn shift4() {
        let mut row = vec! [ None, None, Some(1), None ];
        assert_eq!(Game::shift_row_left(&mut row),
                   vec![ Some(1), None, None, None ]);
    }
                    
    #[test]
    fn shift5() {
        let mut row = vec! [ Some(1), Some(2), None, None ];
        assert_eq!(Game::shift_row_left(&mut row),
                   vec![ Some(1), Some(2), None, None ]);
    }

    #[test]
    fn shift_right() {
        let mut game = Game::new(thread_rng());
        game.board = vec!(vec!(Some(1), None, Some(1), None),
                          vec!(None, Some(1), None, None),
                          vec!(None, None, None, None),
                          vec!(Some(1), Some(2), Some(1), Some(2)));

        let expected = vec!(vec!(None, None, None, Some(2)),
                            vec!(None, None, None, Some(1)),
                            vec!(None, None, None, None),
                            vec!(Some(1), Some(2), Some(1), Some(2)));
        assert_eq!(expected, game.shift(Dir::Right).board);
    }

    #[test]
    fn transpose() {
        let expected = vec!(vec!(Some(1), Some(2), Some(3), Some(4)),
                            vec!(Some(5), Some(6), Some(7), Some(8)));
        assert_eq!(expected,
                   Game::transpose(&vec!(vec!(Some(1), Some(5)),
                                         vec!(Some(2), Some(6)),
                                         vec!(Some(3), Some(7)),
                                         vec!(Some(4), Some(8)))));
    }
}
