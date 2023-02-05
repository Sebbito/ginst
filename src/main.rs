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
pub mod controller;

use std::env;
use types::{Command, Shell};
use clap::Parser;

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
    shell: Option<Shell>,
}


fn main() {
    if cfg!(debug_assertions) {
        env::set_var("RUST_BACKTRACE", "1");
    }

    let args = Arguments::parse();
    controller::handle_arguments(args).unwrap();
}
