extern crate rand;

use rand::Rng;

fn print_board(board: &[[i8; 4]; 4]) {
    println!("+---+---+---+---+");
    for i in board.iter() {
        // TODO: add padding for aligning with double-digit numbers
        println!("| {} | {} | {} | {} |", i[0],i[1],i[2],i[3]);
        println!("+---+---+---+---+");
    }    
}

fn count_empty(board: &[[i8; 4]; 4]) -> i8 {
    let mut total: i8 = 0;
    for row in board.iter() {
        for elem in row {
            if *elem == 0 {
                total += 1;
            }
        }
    }
    return total;
}

fn main() {
    let board = [[0,0,0,0],
                 [0,0,0,0],
                 [0,0,0,0],
                 [0,0,0,0]];
    print_board(&board);
    println!("{}", count_empty(&board));
}
