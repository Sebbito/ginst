use std::fs;
use std::process::Command;

#[derive(Debug)]
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

    println!("{0:#?}", status);
    if status.success() {
        Status::Installed
    } else {
        Status::Missing
    }
}

fn main() {
    let file_contents = fs::read_to_string("programms.json").unwrap();
    let json_parsed = json::parse(&file_contents).expect("Could not parse json file. Maybe you forgot a comma somewhere?");
    let os = "Fedora";

    // println!("{:#}",json_parsed);

    let mut programms: Vec<Programm> = vec![];
    for programm in json_parsed["programms"].members() {
        let prog = Programm {
            name: json::stringify(programm["name"].clone()).to_string(),
            install: json::stringify(programm[os]["install"].clone()).to_string(),
            status: is_installed(json::stringify(programm["name"].clone()).to_string()),
        };

        programms.push(prog)
    }

    println!("{0:#?}", programms);
    println!("{}",programms[0].name)
}
