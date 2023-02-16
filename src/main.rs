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

pub mod program;
pub mod distro;
pub mod parser;
pub mod executor;
pub mod types;
pub mod display;

use std::env;
use clap::Parser;
use program::Program;
use crate::types::{Command, FileType};
use std::{path::Path, error::Error};

/// Args struct holding the CL args
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    /// execute quick operations on programs and exit
    #[command(subcommand)]
    command: Option<Command>,

    /// Path to the file holding program information
    file: String,
    
    /// count all programs (including dependencies)
    #[arg(long, group = "cli")]
    count: bool,
    
    /// count all missing programs (including dependencies)
    #[arg(long, group = "cli")]
    count_missing: bool,
    
    /// perform checks on all programs and dependencies
    #[arg(long, group = "cli")]
    check: bool,

    /// The shell in which the command shall be executed
    #[arg(value_enum)]
    shell: Option<String>,
}


fn main() -> Result<(), Box<dyn Error>> {
    if cfg!(debug_assertions) {
        env::set_var("RUST_BACKTRACE", "1");
    }

    let args = Arguments::parse();
    let shell = executor::eval_shell(args.shell);
    env::set_var("EXECUTE_SHELL", shell);
    let file = &args.file;
    let programs: Vec<Program>= parser::get_programs_from_file(file);

    if args.count {
        println!("{}", program::count(&programs));
    } else if args.count_missing {
        println!("{}", program::count_missing(&programs));
    } else if args.check {
        // parser already ran
        println!("File looks good!");
    } else if let Some(command) = args.command {
        match &command {
            Command::Install { all } => {
                if *all {
                    program::install_missing(&programs);
                }
            },
            Command::Configure { all } => {
                if *all {
                    program::configure_all(&programs);
                }
            },
            Command::Export { filetype } => {
                match filetype {
                    FileType::Json => {
                        let string = serde_json::to_string_pretty(&programs).unwrap();
                        let new_file = Path::new(file).with_extension("json");
                        std::fs::write(new_file, string).unwrap();
                    },
                    FileType::Yaml => {
                        let string = serde_yaml::to_string(&programs).unwrap();
                        let new_file = Path::new(file).with_extension("yml");
                        std::fs::write(new_file, string).unwrap();

                    },
                }
            },
            Command::List { status } => {
                if *status {
                    program::print_status(&programs);
                } else {
                    program::print_name(&programs);
                }
            }
        }
    } else {
        display::run_ui(display::UI::TUI, programs);
    }
    Ok(())
}
