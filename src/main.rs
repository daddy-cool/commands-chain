#![windows_subsystem = "windows"]

use clap::Parser;
use std::os::windows::process::CommandExt;
use std::process::Stdio;

use execute::{command, Execute};

const CREATE_NO_WINDOW: u32 = 0x08000000;

/// Execute a list of commands in sequence
#[derive(Parser, Debug)]
struct Args {
    /// list of commands
    #[arg(num_args(1..))]
    commands: Vec<String>,
}

fn main() {
    #[cfg(windows)]
    {
        use winapi::um::wincon::{AttachConsole, ATTACH_PARENT_PROCESS};
        unsafe {
            AttachConsole(ATTACH_PARENT_PROCESS);
        }
    }
    let args = Args::parse();

    let command_paths = args.commands;

    for command_path in command_paths {
        println!("{:?}", command_path);
        let mut command = command(command_path);

        #[cfg(windows)]
        command.creation_flags(CREATE_NO_WINDOW);

        command.stdout(Stdio::piped());

        let output = command.execute_output().unwrap();

        println!("{}", String::from_utf8(output.stdout).unwrap());
    }
}
