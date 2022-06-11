extern crate dirs;

use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};

/// Function that returns the current directory, or ?
/// if the directory cannot be read.
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

/// Function that acts like above, but instead
/// returns the value of $HOME, on whatever system.
fn get_home_dir() -> String {
    match dirs::home_dir() {
        Some(path) => {
            match path.into_os_string().into_string() {
                Ok(val) => val,
                Err(_) => String::from("/")
            }
        },
        None => String::from("/"),
    }
}

/// Utility to print the prompt starting with the working dir.
fn print_prompt() {
    print!("{}> ", get_working_dir());
    stdout().flush().unwrap();
}

fn main() {
    loop {
        print_prompt();

        // Grab the user's full command
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error reading input: {err}");
                continue;
            }
        };

        // grab first value as the command itself
        let mut cmd_iter = input.trim().split_whitespace();
        let exe = match cmd_iter.next() {
            Some(va) => va,
            None => "",
        };

        // parse the rest of the input as arguments
        let args = cmd_iter;

        // handle builtins each as their own match case
        // to see builtins, run a command like `man cd` or `man exit`
        match exe {
            "cd" => {
                // standard implementation defaults to $HOME if no dir is provided
                let default = get_home_dir();
                let target = args.peekable()
                                 .peek()
                                 .map_or(default.clone(), |x| x.to_string());
                let mut root = Path::new(&target);

                // Allow for parsing ~ as a valid path to the home directory
                if root.to_str().unwrap() == "~" {
                    root = Path::new(&default);
                }

                match env::set_current_dir(&root) {
                    Ok(_) => (),
                    Err(e) => eprintln!("Error switching directories: {e}"),
                }
            },
            "exit" => return,
            exe => {

                // spawn a new process for the entered command
                let mut cmd = Command::new(exe);
                cmd.args(args);
                match cmd.spawn() {
                    Ok(mut proc) => { proc.wait().expect("Command not running."); },
                    Err(e) => eprintln!("Error spawning thread: {e}"),
                }
            },
        }
    }
}
