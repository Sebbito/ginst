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
use std::{io, env, time::Duration, error::Error};
use clap::Parser;

/// Args struct holding the CL args
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the file holding program information
    file: String,
    /// count all programs (including dependencies)
    #[arg(long)]
    count: bool,
    /// execute installation for all programs and exit
    #[arg(long)]
    install: bool,
    /// execute configuration for all programs and exit
    #[arg(long)]
    configure: bool,
    /// List all programs contained in file
    #[arg(long)]
    list: bool,
    /// perform checks on all programs and dependencies
    #[arg(long)]
    check: bool,

}

fn main() -> Result<(), Box<dyn Error>> {
    if cfg!(debug_assertions) {
        env::set_var("RUST_BACKTRACE", "1");
    }

    let args = Args::parse();
    let programs: Vec<program::Program>= parser::get_programs_from_file(args.file);

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

    Ok(())
}
