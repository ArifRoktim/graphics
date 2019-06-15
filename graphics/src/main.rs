use lib_graphics::IDENTITY;
use lib_graphics::screen::ScreenBuilder;
use std::error::Error;
use std::{env, process};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments! Provide a script file!");
        process::exit(1);
    }
    let filename = &args[1];

    let mut screen = ScreenBuilder::default();
    // Default xres and yres is 500
    screen.xres = 500;
    screen.yres = 500;
    // Example:
    //screen.xres = 723;
    //screen.yres = 501;

    let mut screen = screen.create();
    let mut cstack = vec![IDENTITY];

    //parse_graphics::file(filename)?.run(&mut screen, &mut cstack);
    parse_graphics::file(filename)
        // TODO FIXME: Fix this UGLY HIDEOUS HACK
        .unwrap_or_else(|err| {
            let err = err.to_string().replace("NEWLINE", "\n"); panic!("{}", err)
        })
        .run(&mut screen, &mut cstack);
    Ok(())
}
