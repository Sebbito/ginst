//! # ginst
//!
//! ginst is a command line utility used to read and execute installation and configuration
//! commands from specified in a JSON file.
//!
//! To get started download and execute the app:
//!
//! ```bash
//! cargo install ginst
//! ginst --file /path/to/file.json
//! ```
//!
//! For more information see the [ginst Wiki](https://github.com/Sebbito/ginst/wiki)

pub mod display;
pub mod distro;
pub mod executor;
pub mod parser;
pub mod program;
pub mod types;
pub mod commands;

use commands::Command;
use types::{FileType, Programable};
use clap::Parser;
use program::Program;
use std::env;
use std::{error::Error, path::Path};

/// Args struct holding the CL args
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    /// execute quick operations on programs and exit
    #[command(subcommand)]
    command: Option<Command>,

    /// Path to the file holding program information
    file: String,

    /// Start an interactive Terminal User Interface
    #[arg(short, long, group = "cli")]
    interactive: bool,

    /// count all programs (including dependencies)
    #[arg(long, group = "cli")]
    count: bool,

    /// count all missing programs (including dependencies)
    #[arg(long, group = "cli")]
    count_missing: bool,

    /// perform checks if file is syntax is ok
    #[arg(long, group = "cli")]
    check: bool,

    /// list all the programs contained in the file
    #[arg(long, group = "cli")]
    list: bool,

    /// list programs and check if they are installed
    #[arg(long, group = "cli")]
    status: bool,

    /// The shell in which the command shall be executed
    #[arg(value_enum)]
    shell: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // output debug info if the build is a debug build
    if cfg!(debug_assertions) {
        env::set_var("RUST_BACKTRACE", "1");
    }
    // Parse arguments
    let args = Arguments::parse();

    // Check if we can spawn processes with user suggested shell or just use `sh`
    let shell = executor::eval_shell(args.shell);
    // set this as a global variable via environment variables so that future executors can access this easier
    // TODO: find a better way to use these "dynamic global variables"
    env::set_var("EXECUTE_SHELL", shell);

    // open file and parse the file contents
    let file = &args.file;
    let programs: Vec<Program> = parser::get_programs_from_file(file);

    if args.count {
        println!("{}", program::count(&programs));
    } else if args.count_missing {
        println!("{}", program::count_missing(&programs));
    } else if args.check {
        // parser already ran and succeeded
        println!("File looks good!");
    } else if args.list {
        program::print_name(&programs);
    } else if args.status {
        program::print_status(&programs);
    } else if args.interactive {
        display::run_ui(display::UI::TUI, programs);
    } else if let Some(command) = args.command {
        match &command {
            Command::Install { all, program } => {
                if *all {
                    program::install_all(&programs);
                } else if let Some(program_name) = program {
                    // user only wants to install one certain program
                    if let Some(prog) = program::search_from_name(program_name, &programs){
                        prog.install();
                    } else {
                        println!("No program with name {} found", program_name);
                    }
                } else {
                    panic!("No option on install");
                }
            }
            Command::Configure { all, program } => {
                if *all {
                    program::configure_all(&programs);
                } else if let Some(program_name) = program {
                    // user only wants to configure one certain program
                    if let Some(prog) = program::search_from_name(program_name, &programs){
                        prog.configure();
                    } else {
                        println!("No program with name {} found", program_name);
                    }
                } else {
                    panic!("No option on configure");
                }
            }
            Command::Export { filetype } => match filetype {
                FileType::Json => {
                    let string = serde_json::to_string_pretty(&programs).unwrap();
                    let new_file = Path::new(file).with_extension("json");
                    std::fs::write(new_file, string).unwrap();
                }
                FileType::Yaml => {
                    let string = serde_yaml::to_string(&programs).unwrap();
                    let new_file = Path::new(file).with_extension("yml");
                    std::fs::write(new_file, string).unwrap();
                }
            },
        }
    } else {
        // user didn't use any known command
        println!("Please specify what to do.");
        println!("See `ginst --help` for help.");
    }
    Ok(())
}
