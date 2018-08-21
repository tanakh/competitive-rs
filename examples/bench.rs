extern crate competitive;

#[macro_use]
use competitive::*;

fn main() {
    input!{
        n: usize,
        v: [usize; n],
    }

    println!("{}", v.iter().sum::<usize>());
}
