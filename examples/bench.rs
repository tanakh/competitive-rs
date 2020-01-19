use competitive::prelude::*;

fn main() {
    input! {
        n: usize,
        v: [usize; n],
    }

    println!("{}", v.iter().sum::<usize>());
}
