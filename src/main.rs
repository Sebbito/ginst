// use std::path::Path;
use std::{fs, io, env};
use std::process::Command;
use clap::Parser;

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
    let file_contents = if path.len() != 0 {
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

    let programms = program::as_vec_from_json(json_parsed);

    clear();
    // println!("{0:#?}", programms.clone());


    println!("Programms installed:\n");
    programms.print_statuses(0);

    if !programms.are_installed() {
        println!("Do you wish to install all missing programms?\n(Y/n)");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Could not read input");

        if input == "\n" || input == "Y" || input == "y" {
            programms.install_missing();
        }
    }
}

// fn config_routine() {
//     let home = env::var("HOME").expect("What the fuck how is there no home var?");
//     let conf_path: String = format!("{}/.config/.git", home);
//     let path = Path::new(&conf_path).canonicalize().expect("Could not resolve path");

//     println!("{0:#?}", path);
//     if  !path.exists() {
//         println!("Git repo not found in config folder.");
//     }
// }

fn main() {
    if cfg!(debug_assertions) {
        env::set_var("RUST_BACKTRACE", "1");
    }

    let args = Args::parse();

    programm_routine(get_file_contents(args.file));
    // config_routine();
}
