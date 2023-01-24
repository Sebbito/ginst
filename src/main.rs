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

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::{fs, io, env, time::Duration, error::Error};
use clap::Parser;

pub mod app;
pub mod program;
pub mod distro;

/// Args struct holding the CL args
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Path to the json file holding program information
   #[arg(short, long)]
   file: String,
}

/// Utility function for extracting file contents
fn get_file_contents(path: String) -> String {
    let file_contents = if !path.is_empty() {
        fs::read_to_string(path)
                            .expect("Could not find json file. Make sure you are in a directory where theres also the json file.")
    } else {
        panic!("File argument '{}' invalid", path);
    };

    file_contents
}

fn main() -> Result<(), Box<dyn Error>> {
    if cfg!(debug_assertions) {
        env::set_var("RUST_BACKTRACE", "1");
    }

    let args = Args::parse();
    let json_parsed = json::parse(&get_file_contents(args.file.clone()))
                        .expect("Could not parse json file. Maybe you forgot a comma somewhere?");
    let programs = program::collection_from_json(json_parsed);


    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = app::App::new(programs.programs);
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
