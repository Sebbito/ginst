//! # Program
//!
//! Crate with structs representing programs and common operations for said programs

pub mod steps;

use std::process::{Command, Stdio};
use serde::{Deserialize, Serialize};
use crate::distro::get_dist;
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
    status: Status
}

impl Program {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_dependencies(&self) -> Vec<Program>{
        self.dependencies.clone()
    }

    pub fn is_installed(&self) -> bool {
        self.status == Status::Installed && are_installed(&self.dependencies)
    }

    /// Checks if a program is installed using the `command -v` command.
    pub fn check(&self) -> Status {
        /* Performs a check if the program is installed */
        let status = Command::new("command")
                        .arg("-v")
                        .arg(&self.name)
                        .stdout(Stdio::null())
                        .status()
                        .expect("Failed to execute.");

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
        !self.configuration.is_empty() && self.configuration.len() != 0
    }

    pub fn has_installation_steps(&self) -> bool {
        !self.installation.is_empty() && self.installation.len() != 0
    }

    pub fn has_dependencies(&self) -> bool {
        !self.dependencies.is_empty()
    }

    /// Executes installation instructions for the current distro (uses get_dist()) unless it is
    /// already installed
    pub fn install(&self) {
        if self.is_installed() {
            println!("{} is already installed", self.name);
            return;
        }

        let current_dist = get_dist();
        if self.has_installation_steps() {
            // omg this is so nice
            let installation_steps = steps::steps_for_dist(&self.installation, &current_dist);
            if let Some(steps) = installation_steps {
                steps.execute();
            } else {
                println!("No installation instructions for '{}' given", current_dist);
            }
        } else {
            println!("No installation instructions for program '{}' given.", self.name);
        }
    }

    /// Executes configuration instructions for the current distro (uses get_dist())
    pub fn configure(&self) {
        let current_dist = get_dist();
        if self.has_configuration_steps() {
            // omg this is so nice
            let configuration_steps = steps::steps_for_dist(&self.configuration, &current_dist);
            if let Some(steps) = configuration_steps {
                steps.execute();
            } else {
                println!("No configuration instructions for '{}' given", current_dist);
            }
        } else {
            println!("No configuration instructions for program '{}' given.", self.name);
        }
    }

    pub fn get_status(&self) -> String {
        if self.is_installed() {
            "Installed".to_owned()
        } else {
            "Missing".to_owned()
        }
    }

    pub fn get_status_pretty(&self) -> String {
        if self.is_installed() {
            "🗹 Installed".to_owned()
        } else {
            "⮽ Missing".to_owned()
        }
    }
}

pub fn are_installed(programs: &Vec<Program>) -> bool {
    if !programs.is_empty() {
        for val in programs.clone().iter_mut().map(|d| d.is_installed()) {
            if !val {
                return false;
            }
        }
    }

    true
}

pub fn install_missing(programs: &Vec<Program>) {
    for prog in programs.clone() {
        prog.install();
        if prog.has_dependencies() {
            install_missing(&prog.get_dependencies());
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
        println!("{}, {}", program.get_name(), program.get_status());
        if program.has_dependencies() {
            print_status(&program.get_dependencies());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::get_programs_from_file;

    #[test]
    fn test_has_config() {
        let programs = get_programs_from_file(&"example.yaml".to_owned());
        assert!(programs[0].has_configuration_steps());
    }
    #[test]
    fn test_has_install() {
        let programs = get_programs_from_file(&"example.yaml".to_owned());
        assert!(programs[0].has_installation_steps());
    }

    #[test]
    fn test_has_dependencies() {
        // this test is kinda trash but i'm too tired to make it good
        let programs = get_programs_from_file(&"example.yaml".to_owned());
        assert!(programs[0].has_dependencies());
    }

}
