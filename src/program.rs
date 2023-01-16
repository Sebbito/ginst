use std::process::Command;

#[derive(Default, Debug, PartialEq, Clone)]
pub enum Status {
    Installed,
    #[default] Missing,
}

#[derive(Default, Debug, Clone)]
pub struct Program {
    status: Status,
    name: String,
    install: String,
    dependancies: Vec<Program>,
}

impl Program {
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
                println!("No installation instructions for program '{}' given.", self.name);
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

pub mod util {
    use crate::program::Program;
    use json::JsonValue::{self, Null};
    use fti::get_dist;

    pub fn build_dependacy_list(dependancies: JsonValue) -> Vec<Program> {
        if dependancies != Null {
            // println!("Building dependacies with {0:#?}", dependancies.clone());
            return generate_prog_vec(dependancies);
        } else {
            vec![]
        }
    }

    pub fn generate_prog_vec(json_parsed: JsonValue) -> Vec<Program>{
        let mut programs: Vec<Program> = vec![];
        let os = get_dist();

        for program in json_parsed["programs"].members() {
            // println!("{0:#?}", programm.clone());
            // println!("{0:#?}", os.clone());

            let mut prog: Program = Default::default();

            prog.name = program["name"].clone().to_string();
            prog.install = program["install"][os.clone()].clone().to_string();
            prog.status = prog.check();
            if program["dependancies"] != Null {
                prog.dependancies = build_dependacy_list(program["dependancies"].clone());
            } else {
                prog.dependancies = vec![];
            }

            programs.push(prog);
        }
        
        // println!("{0:#?}", programms.clone());
        return programs;
    }

    pub fn install_missing(programs: Vec<Program>) {
        for prog in programs {
            prog.install();
        }
    }

    pub fn count_missing(programs: Vec<Program>) -> u8 {
        let mut counter = 0;
        for p in programs {
            if !p.is_installed() {
                counter += 1;
            }
        }
        counter
    }

}

pub mod display {
    use crate::program::Program;
    pub fn print_all(programs: Vec<Program>) {
        for prog in programs {
            prog.print_all();
        }
    }
}

