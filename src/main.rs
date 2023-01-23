// use std::path::Path;
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
use std::process::Command;
use clap::Parser;

pub mod app;
pub mod program;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Path to the json file holding program information
   #[arg(short, long)]
   file: String,
}

fn clear() {
    // status() because we need to wait for it to finish
    Command::new("clear").status().expect("Failed to execute");
}

fn get_file_contents(path: String) -> String {
    let file_contents = if !path.is_empty() {
        fs::read_to_string(path)
                            .expect("Could not find json file. Make sure you are in a directory where theres also the json file.")
    } else {
        panic!("File argument '{}' invalid", path);
    };

    file_contents
}

fn programm_routine(file_contents: String) {
    let json_parsed = json::parse(&file_contents)
                        .expect("Could not parse json file. Maybe you forgot a comma somewhere?");
    let programs = program::collection_from_json(json_parsed);
    clear();
    // println!("{0:#?}", programms.clone());

    println!("Programms installed:\n");
    programs.print_statuses(0);

    if !programs.are_installed() {
        println!("Do you wish to install all missing programms?\n(Y/n)");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Could not read input");

        if input == "\n" || input == "Y" || input == "y" {
            programs.install_missing();
        }
    }
}

fn config_routine(file_contents: String) {
    let json_parsed = json::parse(&file_contents)
                        .expect("Could not parse json file. Maybe you forgot a comma somewhere?");
    let mut programs = program::collection_from_json(json_parsed);
    clear();

    programs.programs.retain(|p| p.has_configuration_steps());

    println!("Configurations for the following programs found:\n");

    let mut counter = 1;
    for prog in programs.programs.clone() {
        print!("{}: ", counter);
        prog.print_status();
        counter += 1;
    }

    println!("For which of these do you wish to execute the configuration steps?\n");
    let mut index = 1;
    print!("(");
    for _ in 0..programs.len() {
        print!(" {} ", index);
        index += 1;
    }
    println!("): ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Could not read input");

    let nr = input.replace(['\n', '"'], "").parse::<u64>().unwrap() as usize;

    if nr <= programs.len() {
        programs.programs[nr-1].configure();
    }
}

// fn dotfile_routine() {
//     let home = env::var("HOME").expect("What the fuck how is there no home var?");
//     let conf_path: String = format!("{}/.config/.git", home);
//     let path = Path::new(&conf_path).canonicalize().expect("Could not resolve path");

//     // println!("{0:#?}", path);
//     if  !path.exists() {
//         println!("Git repo not found in config folder.");
//     }
// }

fn main() -> Result<(), Box<dyn Error>> {
    if cfg!(debug_assertions) {
        env::set_var("RUST_BACKTRACE", "1");
    }

    let args = Args::parse();
    let json_parsed = json::parse(&get_file_contents(args.file.clone()))
                        .expect("Could not parse json file. Maybe you forgot a comma somewhere?");
    let programs = program::collection_from_json(json_parsed);

    // programm_routine(get_file_contents(args.file.clone()));
    // config_routine(get_file_contents(args.file));

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = app::App::new(programs.programs);
    let res = app::run_app(&mut terminal, app, tick_rate);

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
