use std::env;
use std::collections::VecDeque;
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
        };

        let mut cmd_iter = input.trim().split_whitespace();
        let exe = match cmd_iter.next() {
            Some(va) => va,
            None => "",
        };

        let args = cmd_iter;
        let mut cmd = Command::new(exe);
        cmd.args(args);

        if let Ok(mut proc) = cmd.spawn() {
            proc.wait().expect("Command {exe} wasn't running.");
        } else {
            eprintln!("{exe} command didn't start.");
        }
    }
}
