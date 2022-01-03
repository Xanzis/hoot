use crate::parse::SchemeParserValue;

pub mod build;
mod builtin;
mod env;
mod interpret;

pub fn run(input: SchemeParserValue) {
    let (mut env, prog) = build::build_environment(input);

    println!("Built Environment:\n{}\nProgram Object:\n\t{:?}", env, prog);

    let res = interpret::eval(&mut env, prog);

    println!("Program Result:\n\t{:?}", env.get_object(res));
}
