use std::env;
use std::process;
use logic::arg;

use crate::logic::save;
mod logic;

fn main() {
    let args: Vec<String> = env::args().collect();
    let struct_and_name = arg(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1)
    });
    let _ = save(struct_and_name);
}
