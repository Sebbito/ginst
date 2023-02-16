//! # Program
//!
//! Crate with structs representing programs and common operations for said programs

pub mod steps;

use serde::{Deserialize, Serialize};
use crate::{distro::get_dist, types::{Programable, Sublistable}, executor::Executor};
use self::steps::Steps;

/// Struct indicating the programs installation status
#[derive(Default, Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Status {
    Installed,
    #[default] Missing,
}

/// Struct representing a program 
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    name: String,
    installation: Vec<Steps>,
    configuration: Vec<Steps>,
    dependencies: Vec<Program>,

    /// status will be determined at runtime
    #[serde(skip)]
    status: Status,
}

impl Program {
    pub fn get_dependencies(&self) -> Vec<Program>{
        self.dependencies.clone()
    }

    /// Checks if a program is installed using `type`.
    pub fn check(&self) -> Status {
        // Performs a check if the program is installed
        // use type since it also finds builtins like fisher on fish
        let command = format!("type {}", self.name);
        let status = Executor::new().execute(command).unwrap();

        if status.success() {
            Status::Installed
        } else {
            Status::Missing
        }
    }

    /// sets the status for itself **and all dependencies**
    pub fn set_status(&mut self) {
        self.status = self.check();
        for dep in self.dependencies.iter_mut().by_ref() {
            dep.set_status();
        }
    }

    pub fn has_configuration_steps(&self) -> bool {
        !self.configuration.is_empty()
    }

    pub fn has_installation_steps(&self) -> bool {
        !self.installation.is_empty()
    }

    pub fn has_dependencies(&self) -> bool {
        !self.dependencies.is_empty()
    }

    fn steps_for_current_dist(&self, steps: &Vec<Steps>) -> Option<Steps>{
        let current_dist = get_dist();

        if !steps.is_empty() {
            steps::steps_for_dist(steps.clone(), &current_dist)
        } else {
            println!("Instructions are empty");
            None
        }
    }

    pub fn get_status(&self) -> String {
        if self.is_installed() {
            "Installed".to_owned()
        } else {
            "Missing".to_owned()
        }
    }
}

impl Sublistable for Program {
    fn get_sublist(&self) -> Vec<Program> {
        self.get_dependencies()
    }
}

impl Programable for Program {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Executes installation instructions for the current distro (uses get_dist()) unless it is
    /// already installed
    fn install(&self) {
        let instructions = &self.installation;
        if self.is_installed() {
            println!("{} is already installed", self.name);
        } else {
            if !are_installed(&self.dependencies) {
                install_missing(&self.dependencies);
            }

            if let Some(steps) = self.steps_for_current_dist(instructions) {
                steps.execute();
            }
        }

    }

    /// Executes configuration instructions for the current distro (uses get_dist())
    fn configure(&self) {
        let instructions = &self.configuration;

        if let Some(steps) = self.steps_for_current_dist(instructions) {
            steps.execute();
        }
    }

    fn is_installed(&self) -> bool {
        self.status == Status::Installed && are_installed(&self.dependencies)
    }
}

pub fn are_installed(programs: &Vec<Program>) -> bool {
    if !programs.is_empty() {
        for is_installed in programs.clone().iter_mut().map(|d| d.is_installed()) {
            if !is_installed {
                return false;
            }
        }
    }

    true
}

pub fn install_missing(programs: &Vec<Program>) {
    for prog in programs.clone() {
        if !prog.is_installed() {
            if prog.has_dependencies() {
                install_missing(&prog.get_dependencies());
            }
            prog.install();
        }
    }
}

pub fn configure_all(programs: &Vec<Program>) {
    for prog in programs.clone() {
        prog.configure();
        if prog.has_dependencies() {
            configure_all(&prog.get_dependencies());
        }
    }
}

pub fn count_missing(programs: &Vec<Program>) -> u8 {
    let mut counter = 0;
    for program in programs {
        if !program.is_installed() {
            counter += 1;
        }
        if program.has_dependencies() {
            counter += count_missing(&program.get_dependencies());
        }
    }
    counter
}

pub fn count(programs: &Vec<Program>) -> u8 {
    let mut counter = 0;
    for program in programs {
        counter += 1;
        if program.has_dependencies() {
            counter += count(&program.get_dependencies());
        }
    }
    counter
}

/// Prints name of all programs and their dependencies name
pub fn print_name(programs: &Vec<Program>) {
    for program in programs {
        println!("{}", program.get_name());
        if program.has_dependencies() {
            print_name(&program.get_dependencies());
        }
    }
}

pub fn print_status(programs: &Vec<Program>) {
    for program in programs {
        println!("{},{}", program.get_name(), program.get_status());
        if program.has_dependencies() {
            print_status(&program.get_dependencies());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::get_programs_from_file;

    const PATH: &str = "examples/example.yaml";

    #[test]
    fn test_has_config() {
        let programs = get_programs_from_file(PATH);
        assert!(programs[0].has_configuration_steps());
    }
    #[test]
    fn test_has_install() {
        let programs = get_programs_from_file(PATH);
        assert!(programs[0].has_installation_steps());
    }

    #[test]
    fn test_has_dependencies() {
        // this test is kinda trash but i'm too tired to make it good
        let programs = get_programs_from_file(PATH);
        assert!(programs[0].has_dependencies());
    }

}
