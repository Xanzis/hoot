use std::env;

pub mod parse;
pub mod scheme;

fn main() {
    let input = env::args().nth(1).unwrap();

    println!("{:?}", parse::parse(&input));
}
