use std::{fs, io, env};
use std::process::Command;
use json::JsonValue::{self, Null};

#[derive(Default, Debug, PartialEq, Clone)]
enum Status {
    Installed,
    #[default] Missing,
}

#[derive(Default, Debug, Clone)]
struct Programm {
    status: Status,
    name: String,
    install: String,
    dependancies: Vec<Programm>,
}

impl Programm {
    fn is_installed(&self) -> bool {
        if self.status == Status::Installed && self.dependancies_installed() {
            true
        } else {
            false
        }
    }

    fn check(&self) -> Status {
        let status = Command::new("command")
                        .args(["-v", &self.name])
                        .status()
                        .expect("Failed to execute.");

        if status.success() {
            Status::Installed
        } else {
            Status::Missing
        }
    }

    fn dependancies_installed(&self) -> bool {
        let mut ret = true;

        if !self.dependancies.is_empty() {
            for val in self.dependancies.clone().iter_mut().map(|d| d.is_installed()) {
                if val == false {
                    ret = false;
                }
            }
        }
        ret
    }
    
    fn install(&self) {
        if self.status == Status::Missing {
            // println!("{0:#?}", prog.clone());

            if self.install != "null" {
                let install: Vec<&str> = self.install.split(" ").collect();
                let command = install[0].clone();
                let args: Vec<&str>= install.into_iter().skip(1).collect();

                // println!("{0:#?}", command);
                // println!("{0:#?}", args);

                let status = Command::new(command).args(args).status().expect("Failed to execute install command");
                if !status.success() {
                    panic!("Something is fucky wucky");
                }
            } else {
                println!("No installation instructions for programm '{}' given.", self.name);
            }
        }
    }

    fn print_all(&self) {
        self.print_status();
        self.print_dependacies();
    }

    fn print_status(&self) {
        if self.is_installed() {
            println!("[✓] {}", self.name)
        } else {
            println!("[⤫] {}", self.name)
        }
    }

    fn print_dependacies(&self) {
        for dep in self.dependancies.clone() {
            print!("  ");
            dep.print_status();
        }
    }
}

fn print_all(programms: Vec<Programm>) {
    for prog in programms {
        prog.print_all();
    }
}


fn build_dependacy_list(dependancies: JsonValue) -> Vec<Programm> {
    if dependancies != Null {
        // println!("Building dependacies with {0:#?}", dependancies.clone());
        return generate_prog_vec(dependancies);
    } else {
        vec![]
    }
}

fn generate_prog_vec(json_parsed: JsonValue) -> Vec<Programm>{
    let mut programms: Vec<Programm> = vec![];
    let os = get_dist();

    for programm in json_parsed["programms"].members() {
        // println!("{0:#?}", programm.clone());
        // println!("{0:#?}", os.clone());

        let mut prog: Programm = Default::default();

        prog.name = programm["name"].clone().to_string();
        prog.install = programm[os.clone()]["install"].clone().to_string();
        prog.status = prog.check();
        if programm["dependancies"] != Null {
            prog.dependancies = build_dependacy_list(programm["dependancies"].clone());
        } else {
            prog.dependancies = vec![];
        }

        programms.push(prog);
    }
    
    // println!("{0:#?}", programms.clone());
    return programms;
}

fn get_dist() -> String {
    let output = Command::new("grep")
                    .args(["^NAME=\".*\"","/etc/os-release"])
                    .output()
                    .expect("Failed to get os information");
    let raw = String::from_utf8(output.stdout).expect("Failed to generate string from stoud");

    return raw.replace("\"", "").replace("\n", "").chars().skip(5).collect();
}

fn clear() {
    // status() because we need to wait for it to finish
    Command::new("clear").status().expect("Failed to execute");
}

fn install_missing(programms: Vec<Programm>) {
    for prog in programms {
        prog.install();
    }
}

fn count_missing(programms: Vec<Programm>) -> u8 {
    let mut counter = 0;
    for p in programms {
        if !p.is_installed() {
            counter += 1;
        }
    }
    counter
}

fn main() {
    let file_contents = fs::read_to_string("programms.json").unwrap();
    let json_parsed = json::parse(&file_contents).expect("Could not parse json file. Maybe you forgot a comma somewhere?");

    let programms = generate_prog_vec(json_parsed);
    env::set_var("RUST_BACKTRACE", "full");

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
