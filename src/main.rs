extern crate dirs;

use std::env;
use std::collections::VecDeque;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Command, Stdio};

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
fn spawn_command(exe: &str,
                 args: std::str::SplitWhitespace,
                 previous_command: Option<&str>) {
    // spawn a new process for the entered command
    let mut cmd = Command::new(exe);
    cmd.args(args);
    match cmd.spawn() {
        Ok(mut proc) => { proc.wait().expect("Command not running."); },
        Err(e) => eprintln!("Error spawning thread: {e}"),
    }
}

/// Function that handles the `cd` builtin
fn handle_cd(args: std::str::SplitWhitespace,
             stack: &mut VecDeque<String>,
             from_pushd: bool) {
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
            if !from_pushd {
                stack.pop_back();
                stack.push_back(target);
            } else {
                stack.push_back(target);
                handle_dirs(stack);
            }
        },
        Err(e) => eprintln!("Error switching directories: {e}"),
    };
}

/// Implements the `popd` builtin.
/// This is the inverse of pushd, returning to the previous directory
/// on the stack.
fn handle_popd(stack: &mut VecDeque<String>) {
    if stack.len() > 1 {

        let target = match stack.pop_back() {
            Some(v) => v,
            None => String::from("/"), // default to root if no option is there (shouldn't happen)
        };

        let target = Path::new(&target);

        match env::set_current_dir(&target) {
            Ok(_) => handle_dirs(stack),
            Err(e) => eprintln!("Error switching directories: {e}"),
        };
    } else {
        eprintln!("popd: directory stack empty");
    }
}

/// Implements the `dirs` builtin.
/// This function prints out the current directory stack
/// If the directory stack only has 1 value, it functions identically
/// to the `pwd` command
fn handle_dirs(stack: &VecDeque<String>) {
    // need to print in reverse to show proper stack order
    for dir in stack.iter().rev() {
        print!("{dir} ");
    }
    println!();
}

fn main() {
    // Clear the screen just to start
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let mut stack: VecDeque<String> = VecDeque::new();
    stack.push_back(get_working_dir());

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

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command: Option<&str> = None;

        while let Some(exe) = commands.next() {

            // grab first value as the command itself
            let mut cmd_iter = exe.trim().split_whitespace();
            let exe = match cmd_iter.next() {
                Some(va) => va,
                None => "",
            };

            // parse the rest of the input as arguments
            let args = cmd_iter;

            // handle builtins each as their own match case
            // to see builtins, run a command like `man cd` or `man exit`
            match exe {
                "exit" => return,
                "cd" => {
                    handle_cd(args.clone(), &mut stack, false);
                    previous_command = None;
                },
                "pushd" => {
                    handle_cd(args.clone(), &mut stack, true);
                    previous_command = None;
                },
                "popd" => {
                    handle_popd(&mut stack);
                    previous_command = None;
                }
                "dirs" => {
                    handle_dirs(&stack);
                    previous_command = None;
                },
                exe => spawn_command(exe, args, previous_command),
            };
        }
    }
}
