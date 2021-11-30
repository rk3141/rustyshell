use owo_colors::OwoColorize;
use std::env;

pub fn cd(args: Vec<&str>) {
    let dir = args.first().unwrap();
    env::set_current_dir(dir).expect("couldnt change director");
    print!("{} {}", "Changed directory to".green(), dir.yellow());
}
