use lib_graphics::{parse, Screen, IDENTITY};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments! Provide a script file!");
        process::exit(1);
    }
    let filename = &args[1];

    let mut screen = Screen::default();
    let mut cstack = vec![IDENTITY];

    parse::parse_file(filename, &mut screen, &mut cstack);
}
