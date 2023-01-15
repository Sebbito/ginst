use std::fs;
use std::process::Command;

use json::JsonValue;

#[derive(Debug, PartialEq)]
enum Status {
    Installed,
    Missing,
}

#[derive(Debug)]
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
    raw.replace("\"", "").chars().skip(5).collect()
}

fn clear() {
    Command::new("clear").status().expect("Failed to execute");
}

fn main() {
    let file_contents = fs::read_to_string("programms.json").unwrap();
    let json_parsed = json::parse(&file_contents).expect("Could not parse json file. Maybe you forgot a comma somewhere?");

    let programms = generate_prog_vec(json_parsed);

    clear();

    println!("Programms installed:\n");
    print_status(programms);
}
