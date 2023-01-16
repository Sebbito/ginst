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
    dependencies: Vec<Program>,
}

impl Program {
    fn is_installed(&self) -> bool {
        if self.status == Status::Installed && self.dependencies_installed() {
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

    fn dependencies_installed(&self) -> bool {
        let mut ret = true;

        if !self.dependencies.is_empty() {
            for val in self.dependencies.clone().iter_mut().map(|d| d.is_installed()) {
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
        for dep in self.dependencies.clone() {
            print!("  ");
            dep.print_status();
        }
    }
}

pub mod util {
    use crate::programm::Program;
    use json::JsonValue::{self, Null};
    use fti::get_dist;

    pub fn build_dependacy_list(dependencies: JsonValue) -> Vec<Program> {
        if dependencies != Null {
            // println!("Building dependacies with {0:#?}", dependencies.clone());
            return generate_prog_vec(dependencies);
        } else {
            vec![]
        }
    }

    pub fn generate_prog_vec(json_parsed: JsonValue) -> Vec<Program>{
        let mut programms: Vec<Program> = vec![];
        let os = get_dist();

        for programm in json_parsed["programs"].members() {
            // println!("{0:#?}", programm.clone());
            // println!("{0:#?}", os.clone());

            let mut prog: Program = Default::default();

            prog.name = programm["name"].clone().to_string();
            prog.install = programm["install"][os.clone()].clone().to_string();
            prog.status = prog.check();
            if programm["dependencies"] != Null {
                prog.dependencies = build_dependacy_list(programm["dependencies"].clone());
            } else {
                prog.dependencies = vec![];
            }

            programms.push(prog);
        }
        
        // println!("{0:#?}", programms.clone());
        return programms;
    }

    pub fn install_missing(programms: Vec<Program>) {
        for prog in programms {
            prog.install();
        }
    }

    pub fn count_missing(programms: Vec<Program>) -> u8 {
        let mut counter = 0;
        for p in programms {
            if !p.is_installed() {
                counter += 1;
            }
        }
        counter
    }

}

pub mod display {
    use crate::programm::Program;
    pub fn print_all(programms: Vec<Program>) {
        for prog in programms {
            prog.print_all();
        }
    }
}


