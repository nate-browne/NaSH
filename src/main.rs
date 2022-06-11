extern crate dirs;

use std::env;
use std::collections::VecDeque;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::Command;

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
/// Will default to the root directory `/` if home couldn't be
/// processed.
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

/// Function that spawns a process for a non-builtin command
fn spawn_command(exe: &str, args: std::str::SplitWhitespace) {
    // spawn a new process for the entered command
    let mut cmd = Command::new(exe);
    cmd.args(args);
    match cmd.spawn() {
        Ok(mut proc) => { proc.wait().expect("Command not running."); },
        Err(e) => eprintln!("Error spawning thread: {e}"),
    }
}

/// Function that handles the `cd` builtin
fn handle_cd(args: std::str::SplitWhitespace, stack: &mut VecDeque<String>) {
    // standard implementation defaults to $HOME if no dir is provided
    let default = get_home_dir();
    let mut target = args.peekable()
                        .peek()
                        .map_or(default.clone(), |x| x.to_string());

    // Cool idea to replace the `~` with what it evaluates as ($HOME)
    target = target.replace("~", default.as_str());

    let root = Path::new(&target);

    match env::set_current_dir(&root) {
        // This is needed to update the directory stack
        Ok(_) => {
            stack.pop_back();
            stack.push_back(target);
        },
        Err(e) => eprintln!("Error switching directories: {e}"),
    }
}

/// Implements the `pushd` builtin.
/// This builtin goes to a new directory, adding it to the
/// top of a directory stack.
fn handle_pushd(stack: &mut VecDeque<String>, args: std::str::SplitWhitespace) {}

/// Implements the `popd` builtin.
/// This is the inverse of pushd, returning to the previous directory
/// on the stack.
fn handle_popd(stack: &mut VecDeque<String>, args: std::str::SplitWhitespace) {}

/// Implements the `dirs` builtin.
/// This function prints out the current directory stack
/// If the directory stack only has 1 value, it functions identically
/// to the `pwd` command
fn handle_dirs(stack: &VecDeque<String>, args: std::str::SplitWhitespace) {
    for dir in stack {
        print!("{dir} ");
    }
    println!();
}

fn main() {
    // Clear the screen just to start
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let mut dir_stk: VecDeque<String> = VecDeque::new();
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
            "cd" => handle_cd(args, &mut dir_stk),
            "exit" => return,
            "pushd" => handle_pushd(&mut dir_stk, args),
            "popd" => handle_popd(&mut dir_stk, args),
            "dirs" => handle_dirs(&dir_stk, args),
            exe => spawn_command(exe, args),
        }
    }
}
