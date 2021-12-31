pub mod scheme;
pub mod parse;

fn main() {
    let test = "hello-world";

    println!("{:?}", parse::parse(test));
}
