
use std::{env};
mod bf3d;

fn main() {
    let input = std::fs::read_to_string(&env::args().collect::<Vec<String>>()[1])
        .expect("Couldn't read file. Give a file with bf3d code as argument");
    bf3d::bf3d(input);
}
