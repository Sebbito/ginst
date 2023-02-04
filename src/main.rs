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

pub mod app;
pub mod program;
pub mod distro;
pub mod parser;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::{io, env, time::Duration, error::Error, path::Path};
use clap::{ArgGroup, Parser, Subcommand, ValueEnum};

/// Args struct holding the CL args
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(group(
            ArgGroup::new("cli")
                .required(false)
                .multiple(false)
                .args(["count", "count_missing", "list", "list"]),
        ))]
struct CLI {
    /// execute quick operations on programs and exit
    #[command(subcommand)]
    command: Option<Command>,

    /// Path to the file holding program information
    file: String,
    
    /// count all programs (including dependencies)
    #[arg(long)]
    count: bool,
    
    /// count all missing programs (including dependencies)
    #[arg(long)]
    count_missing: bool,
    
    /// List all programs contained in file
    #[arg(long)]
    list: bool,
    
    /// perform checks on all programs and dependencies
    #[arg(long)]
    check: bool,
}

#[derive(Subcommand)]
enum Command {
    Install {
        #[arg(long, short)]
        /// install all (missing)
        all: bool,
    },

    Configure {
        #[arg(long, short)]
        /// configure all
        all: bool,
    },

    Export {
        #[arg(value_enum)]
        filetype: FileType
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum FileType {
    Json,
    Yaml,
}

fn main() -> Result<(), Box<dyn Error>> {
    if cfg!(debug_assertions) {
        env::set_var("RUST_BACKTRACE", "1");
    }

    let cli = CLI::parse();
    let file = &cli.file;
    let programs: Vec<program::Program>= parser::get_programs_from_file(&cli.file);

    if cli.count {
        println!("{}", program::count(&programs));
    } else if cli.count_missing {
        println!("{}", program::count_missing(&programs));
    } else if cli.list {
        program::print_name(&programs);
    } else if cli.check {
        // parser already ran
        println!("File looks good!");
    } else if let Some(command) = cli.command {
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
                        let new_file = Path::new(&file).with_extension("json");
                        std::fs::write(new_file, string).unwrap();
                    },
                    FileType::Yaml => {
                        let string = serde_yaml::to_string(&programs).unwrap();
                        let new_file = Path::new(&file).with_extension("json");
                        std::fs::write(new_file, string).unwrap();

                    },
                }
            }
        }
    } else {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // create app and run it
        let tick_rate = Duration::from_millis(250);
        let app = app::App::new(programs);
        let res = app::run_app(&mut terminal, app, tick_rate, false);

        // restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            println!("{:?}", err)
        }
    }

    Ok(())
}
