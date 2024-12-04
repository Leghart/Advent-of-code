mod day4;
use day4::code::{part1, part2};

fn main() {
    match part1() {
        Some(x) => println!("Result part1: {}", x),
        None => println!("Result part1: ---"),
    }

    match part2() {
        Some(x) => println!("Result part2: {}", x),
        None => println!("Result part2: ---"),
    }
}
