use lib_graphics::{Screen, IDENTITY};
use std::{env, process};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments! Provide a script file!");
        process::exit(1);
    }
    let filename = &args[1];

    let mut screen = Screen::default();
    let mut cstack = vec![IDENTITY];

    parse_graphics::file(filename)?.run(&mut screen, &mut cstack);
    Ok(())
}
