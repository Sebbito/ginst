use std::{fs, io, env};
use std::process::Command;

use json::JsonValue;

#[derive(Debug, PartialEq, Clone)]
enum Status {
    Installed,
    Missing,
}

#[derive(Debug, Clone)]
struct Programm {
    status: Status,
    name: String,
    install: String,
}

fn is_installed(programm: String) -> Status {

    let status = Command::new("command")
                    .args(["-v", &programm])
                    .status()
                    .expect("Failed to execute.");

    if status.success() {
        Status::Installed
    } else {
        Status::Missing
    }
}

fn print_status(programms: Vec<Programm>) {
    for programm in programms.iter() {
        if programm.status == Status::Missing {
            println!("[⤫] {}", programm.name)
        } else {
            println!("[✓] {}", programm.name)
        }
    }
}

fn generate_prog_vec(json_parsed: JsonValue) -> Vec<Programm> {
    let mut programms: Vec<Programm> = vec![];
    let os = get_dist();

    for programm in json_parsed["programms"].members() {
        // println!("{0:#?}", programm.clone());
        // println!("{0:#?}", os.clone());
        

        let prog = Programm {
            name: programm["name"].clone().to_string(),
            install: programm[os.clone()]["install"].clone().to_string(),
            status: is_installed(programm["name"].clone().to_string()),
        };
        programms.push(prog)
    }
    programms
}

fn get_dist() -> String {
    let output = Command::new("grep")
                    .args(["^NAME=\".*\"","/etc/os-release"])
                    .output()
                    .expect("Failed to get os information");

    let raw = String::from_utf8(output.stdout).expect("Failed to generate string from stoud");
    raw.replace("\"", "").replace("\n", "").chars().skip(5).collect()
}

fn clear() {
    Command::new("clear").status().expect("Failed to execute");
}

fn install_missing(programms: Vec<Programm>) {
    for prog in programms {

        if prog.status == Status::Missing {
            // println!("{0:#?}", prog.clone());

            if prog.install != "null" {
                let install: Vec<&str> = prog.install.split(" ").collect();
                let command = install[0].clone();
                let args: Vec<&str>= install.into_iter().skip(1).collect();

                // println!("{0:#?}", command);
                // println!("{0:#?}", args);

                let status = Command::new(command).args(args).status().expect("Failed to execute install command");
                if !status.success() {
                    panic!("Something is fucky wucky");
                }
            } else {
                println!("No installation instructions for programm '{}' given.", prog.name);
            }
        }
    }
}

fn main() {
    let file_contents = fs::read_to_string("programms.json").unwrap();
    let json_parsed = json::parse(&file_contents).expect("Could not parse json file. Maybe you forgot a comma somewhere?");

    let programms = generate_prog_vec(json_parsed);
    env::set_var("RUST_BACKTRACE", "full");

    // clear();

    println!("Programms installed:\n");
    print_status(programms.clone());

    println!("Do you wish to install all missing programms?\n\n(Y/n)");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Could not read input");

    println!("{0:#?}", input);
    if input == "\n" || input == "y" || input == "y" {
        install_missing(programms.clone());
    }
}
