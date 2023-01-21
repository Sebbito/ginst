use std::process::Command;
use ginst::get_dist;
use json::JsonValue::{self, Null};

pub mod instructionset;
pub mod steps;

#[derive(Default, Debug, PartialEq, Clone)]
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
    dependencies: ProgramCollection,
}

impl Program {
    fn is_installed(&self) -> bool {
        if self.status == Status::Installed && self.dependencies.are_installed() {
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
    
    fn install(&self) {
        let current_dist = get_dist();
        if self.status == Status::Missing && self.installation.len() != 0 {
            // omg this is so nice
            let installation_steps = self.installation.for_dist(current_dist.clone());
            if installation_steps.is_some() {
                installation_steps.unwrap().execute();
            }
            println!("No installation instructions for '{}' given", current_dist);
        } else {
            println!("No installation instructions for program '{}' given.", self.name);
        }
    }

    fn print(&self, indent_level: u8) {
        self.print_status();
        self.dependencies.print_statuses(indent_level + 1);
    }

    fn print_status(&self) {
        if self.is_installed() {
            println!("[✓] {}", self.name);
        } else {
            println!("[⤫] {}", self.name);
        }
    }
}


#[derive(Default, Debug, Clone)]
pub struct ProgramCollection {
    programs: Vec<Program>
}

impl ProgramCollection {
    pub fn are_installed(&self) -> bool {
        if !self.is_empty() {
            for val in self.programs.clone().iter_mut().map(|d| d.is_installed()) {
                if val == false {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_empty(&self) -> bool {
        self.programs.len() == 0
    }

    pub fn print_statuses(&self, indent_level: u8) {
        for program in self.programs.clone() {
            for _ in 0..indent_level {
                print!("  "); // indent by 1 block
            }
            program.print(indent_level);
        }
    }

    pub fn install_missing(&self) {
        for prog in self.programs.clone() {
            prog.install();
        }
    }

    pub fn count_missing(&self) -> u8 {
        let mut counter = 0;
        for p in self.programs.clone() {
            if !p.is_installed() {
                counter += 1;
            }
        }
        counter
    }

    pub fn push(&mut self, program: Program) {
        self.programs.push(program);
    }

}

pub fn from_json(json_parsed: &JsonValue) -> Program {
    let mut prog: Program = Default::default();

    prog.name = json_parsed["name"].clone().to_string();
    prog.installation = instructionset::from_json(json_parsed["installation"].clone());
    prog.configuration = instructionset::from_json(json_parsed["configuration"].clone());
    prog.status = prog.check();
    prog.dependencies = as_vec_from_json(json_parsed["dependencies"].clone());
    
    prog
}

pub fn as_vec_from_json(json_parsed: JsonValue) -> ProgramCollection{
    let mut programs: ProgramCollection = Default::default();

    if json_parsed != Null {
        for program in json_parsed["programs"].members() {
            programs.push(from_json(program));
        }
    }
    
    return programs;
}
