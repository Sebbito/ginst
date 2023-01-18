use std::process::Command;
use ginst::get_dist;
use json::JsonValue;

pub mod util;
pub mod display;
pub mod instructionset;
pub mod steps;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum Status {
    Installed,
    #[default] Missing,
}

#[derive(Default, Debug, Clone)]
pub struct Program {
    status: Status,
    name: String,
    installation: instructionset::InstructionSet,
    configuration: instructionset::InstructionSet,
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
        /* Performs a check if the program is installed */
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

    pub fn has_configuration_steps(&self) -> bool {
        return self.configuration.len() != 0;
    }

    pub fn has_installation_steps(&self) -> bool {
        return self.installation.len() != 0;
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
    
    pub fn install(&self) {
        let current_dist = get_dist();
        if !self.is_installed() && self.has_installation_steps() {
            // omg this is so nice
            let installation_steps = self.installation.for_dist(current_dist.clone());
            if installation_steps.is_some() {
                installation_steps.unwrap().execute();
            } else {
                println!("No installation instructions for '{}' given", current_dist);
            }
        } else {
            println!("No installation instructions for program '{}' given.", self.name);
        }
    }

    pub fn configure(&self) {
        let current_dist = get_dist();
        if self.has_configuration_steps() {
            // omg this is so nice
            let configuration_steps = self.configuration.for_dist(current_dist.clone());
            if configuration_steps.is_some() {
                configuration_steps.unwrap().execute();
            } else {
                println!("No configuration instructions for '{}' given", current_dist);
            }
        } else {
            println!("No configuration instructions for program '{}' given.", self.name);
        }
    }

    fn print(&self) {
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
            print!("    "); // indent by 1 block
            dep.print_status();
        }
    }
}

pub fn from_json(json_parsed: &JsonValue) -> Program {
    let mut prog: Program = Default::default();

    prog.name = json_parsed["name"].clone().to_string();
    prog.installation = instructionset::from_json(json_parsed["installation"].clone());
    prog.configuration = instructionset::from_json(json_parsed["configuration"].clone());
    prog.status = prog.check();
    prog.dependencies = util::build_dependency_list(json_parsed["dependencies"].clone());
    
    prog
}

pub fn as_vec_from_json(json_parsed: JsonValue) -> Vec<Program>{
    let mut programs: Vec<Program> = vec![];

    for program in json_parsed["programs"].members() {
        programs.push(from_json(program));
    }
    
    return programs;
}
