use std::path::Path;
use std::{fs, io, env};
use std::process::Command;
use program::util;
use program::display::print_all;

pub mod program;

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
    print_all(programms.clone());

    if util::count_missing(programms.clone()) > 0 {
        println!("Do you wish to install all missing programms?\n(Y/n)");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Could not read input");

        if input == "\n" || input == "Y" || input == "y" {
            util::install_missing(programms.clone());
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

fn help() {
    println!("Please supply a path name to the json file.")
}

fn main() {
    // env::set_var("RUST_BACKTRACE", "full");
 let args: Vec<String> = env::args().collect();

    match args.len() {
        // no arguments passed
        1 => {
            println!("Please supply a file path.");
        },
        // one argument passed
        2 => {
            match args[1].parse::<String>() {
                Ok(s) => {
                    programm_routine(get_file_contents(s));
                },
                _ => println!("Invalid path given."),
            }
        },
        // // one command and one argument passed
        // 3 => {
        //     let cmd = &args[1];
        //     let num = &args[2];
        //     // parse the number
        //     let number: i32 = match num.parse() {
        //         Ok(n) => {
        //             n
        //         },
        //         Err(_) => {
        //             eprintln!("error: second argument not an integer");
        //             help();
        //             return;
        //         },
        //     };
        //     // parse the command
        //     match &cmd[..] {
        //         "increase" => increase(number),
        //         "decrease" => decrease(number),
        //         _ => {
        //             eprintln!("error: invalid command");
        //             help();
        //         },
        //     }
        // },
        // all the other cases
        _ => {
            // show a help message
            println!("Invalid or no arguments");
            help();
        }
    }
    config_routine();
}
