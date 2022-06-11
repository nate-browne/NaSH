use std::env;
use std::io::{stdin, stdout, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio, exit};

fn get_working_dir() -> String {
    match env::current_dir() {
        Ok(path) => {
            match path.into_os_string().into_string() {
                Ok(val) => val,
                Err(_) => String::from("?")
            }
        },
        Err(_) => String::from("?"),
    }
}

fn print_prompt() {
    print!("{}> ", get_working_dir());
    stdout().flush().unwrap();
}

fn main() {
    loop {
        print_prompt();

        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error reading input: {err}");
                continue;
            }
        }
        let val = input.trim();

        let mut cmd = Command::new(val);

        if let Ok(mut proc) = cmd.spawn() {
            proc.wait().expect("Command {val} wasn't running.");
        } else {
            eprintln!("{val} command didn't start.");
        }
    }
}
