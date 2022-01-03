use crate::parse::SchemeParserValue;

pub mod build;
mod env;
mod builtin;

pub fn run(input: SchemeParserValue) {
	let (env, prog) = build::build_environment(input);

	println!("Built Environment:\n{}\nProgram Object:\n{:?}", env, prog);
}