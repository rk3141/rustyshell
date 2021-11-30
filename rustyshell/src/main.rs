use std::{
    env,
    io::{stdin, stdout, Write},
    process::Command,
};

use owo_colors::OwoColorize;

fn main() {
    let mut stdout = stdout();
    let stdin = stdin();

    loop {
        let fwd = env::current_dir().unwrap();
        let fwd = fwd.as_os_str().to_str().unwrap();
        let home = env::var("HOME").unwrap();

        let fwd = "~".to_string() + fwd.strip_prefix(&home).unwrap();

        let prompt = format!("{} >", fwd);
        let prompt = prompt.green();

        print!("\x1Bc {} ", prompt);

        stdout.flush().unwrap();
        let mut cmd = String::new();
        stdin.read_line(&mut cmd).unwrap();
        cmd = cmd.trim().to_string();
        if cmd == "" {
            break;
        }

        let argv: Vec<&str> = cmd.split(" ").collect();
        let (prog, args) = argv.split_first().unwrap();
        let args = args.to_vec();

        match *prog {
            "cd" => {
                internals::cd::cd(args);
            }
            "help" => {
                internals::help::help(args);
            }

            _ => {
                if let Ok(mut proc) = Command::new(prog).args(args).spawn() {
                    proc.wait().unwrap();
                } else {
                    println!("{} {}", prog.bright_yellow(), "NOT FOUND".red())
                }
            }
        }

        println!("\nPRESS ENTER TO CONTINUE");
        stdin.read_line(&mut cmd).unwrap();
    }
}
