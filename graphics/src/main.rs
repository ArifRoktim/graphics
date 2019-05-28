use lib_graphics::{Screen, IDENTITY};
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

    let todo = parse_graphics::file(filename);
    match todo {
        Ok(list) => list.run(&mut screen, &mut cstack),
        Err(err) => panic!(err),
    }
}
