extern crate rand;
extern crate fibby;

use rand::thread_rng;
use fibby::{Dir,Game};

fn main() {
    Game::new(thread_rng())
        .shift(Dir::Left)
        .shift(Dir::Right)
        .print();
}
