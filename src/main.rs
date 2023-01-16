use std::path::Path;
use std::{fs, io, env};
use std::process::Command;
use crate::programm::util::{generate_prog_vec, count_missing, install_missing};
use crate::programm::display::print_all;

pub mod programm;

fn clear() {
    // status() because we need to wait for it to finish
    Command::new("clear").status().expect("Failed to execute");
}

fn programm_routine() {
    let file_contents = fs::read_to_string("programms.json")
                            .expect("Could not find json file. Make sure you are in a directory where theres also the json file.");
    let json_parsed = json::parse(&file_contents)
                        .expect("Could not parse json file. Maybe you forgot a comma somewhere?");

    let programms = generate_prog_vec(json_parsed);

    //
    // println!("{0:#?}", programms.clone());
    clear();

    println!("Programms installed:\n");
    print_all(programms.clone());

    if count_missing(programms.clone()) > 0 {
        println!("Do you wish to install all missing programms?\n(Y/n)");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Could not read input");

        // println!("{0:#?}", input);
        if input == "\n" || input == "Y" || input == "y" {
            install_missing(programms.clone());
        }
    }
}

fn config_routine() {
    let home = env::var("HOME").expect("What the fuck how is there no home var?");
    let conf_path: String = format!("{}/.config/.git", home);
    let path = Path::new(&conf_path).canonicalize().expect("Could not resolve path");

    println!("{0:#?}", path);
    if  !path.exists() {
        println!("Git repo not found in config folder.");
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "full");

    programm_routine();
    config_routine();
}
