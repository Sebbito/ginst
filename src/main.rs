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
pub mod executor;
pub mod cli;

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
use cli::{Command, FileType, Shell};
use clap::Parser;

/// Args struct holding the CL args
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CLI {
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
