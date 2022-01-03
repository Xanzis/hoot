use std::env;

pub mod parse;
pub mod scheme;

fn main() {
    let input = env::args().nth(1).unwrap();

    // TODO replace with proper error handling ASAP
    let parsed = parse::parse(&input).unwrap();

    println!("{:?}", parsed);

    scheme::run(parsed);
}
