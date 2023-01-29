//! # Program
//!
//! Crate with structs representing programs and common operations for said programs

pub mod steps;

use std::process::Command;
use crate::distro::get_dist;
use serde::{Deserialize, Serialize};

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
    pub name: String,
    pub installation: Vec<Steps>,
    pub configuration: Vec<Steps>,
    pub dependencies: ProgramCollection,

    #[serde(skip)]
    status: Status
}

impl Program {
    pub fn is_installed(&self) -> bool {
        self.status == Status::Installed && self.dependencies.are_installed()
    }

    /// Checks if a program is installed using the `command -v` command.
    pub fn check(&self) -> Status {
        /* Performs a check if the program is installed */
        let status = Command::new("command")
                        .arg("-v")
                        .arg(&self.name)
                        .status()
                        .expect("Failed to execute.");

        if status.success() {
            Status::Installed
        } else {
            Status::Missing
        }
    }

    pub fn set_status(&mut self) {
        self.status = self.check();
        for dep in self.dependencies.programs.iter_mut().by_ref() {
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
        !self.dependencies.is_empty() && self.dependencies.len() != 0
    }

    /// Utilitiy function returning a Steps struct for the users distribution
    pub fn config_steps_for_dist(&self, dist_name: String) -> Option<Steps> {
        for steps in self.configuration.clone() {
            for dist in steps.distro.clone() {
                if dist == dist_name || dist == "*"{
                    return Some(steps)
                }
            }
        }
        None
    }

    /// Utilitiy function returning a Steps struct for the users distribution
    pub fn install_steps_for_dist(&self, dist_name: String) -> Option<Steps> {
        for steps in self.installation.clone() {
            for dist in steps.distro.clone() {
                if dist == dist_name || dist == "*"{
                    return Some(steps)
                }
            }
        }
        None
    }

    /// Executes installation instructions for the current distro (uses get_dist())
    pub fn install(&self) {
        if self.is_installed() {
            println!("{} is already installed", self.name);
            return;
        }

        let current_dist = get_dist();
        if self.has_installation_steps() {
            // omg this is so nice
            let installation_steps = self.install_steps_for_dist(current_dist.clone());
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
            let configuration_steps = self.config_steps_for_dist(current_dist.clone());
            if let Some(steps) = configuration_steps {
                steps.execute();
            } else {
                println!("No configuration instructions for '{}' given", current_dist);
            }
        } else {
            println!("No configuration instructions for program '{}' given.", self.name);
        }
    }
}

/// A collection of programs with some utilie
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ProgramCollection {
    pub programs: Vec<Program>
}

impl ProgramCollection {

    pub fn len(&self) -> usize {
        self.programs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.programs.is_empty() && self.programs.len() == 0
    }

    pub fn are_installed(&self) -> bool {
        if !self.is_empty() {
            for val in self.programs.clone().iter_mut().map(|d| d.is_installed()) {
                if !val {
                    return false;
                }
            }
        }

        true
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

pub fn from_json_file(path: String) -> Option<ProgramCollection> {
    let prog = std::fs::read_to_string(path).unwrap();

    // use serde niceness
    if let Some(mut result) = serde_json::from_str::<ProgramCollection>(&prog).ok() {
        // now check if each program is installed
        for prog in result.programs.iter_mut().by_ref() {
            prog.set_status();
        }
        return Some(result);
    } else {
        None
    }
}

pub fn from_yaml_file(path: String) -> Option<ProgramCollection> {
    let prog = std::fs::read_to_string(path).unwrap();

    // use serde niceness
    if let Some(mut result) = serde_yaml::from_str::<ProgramCollection>(&prog).ok() {
        // now check if each program is installed
        for prog in result.programs.iter_mut().by_ref() {
            prog.set_status();
        }
        return Some(result);
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::program::from_json_file;

    #[test]
    fn test_has_config() {
        let prog = from_json_file("example.json".to_owned()).unwrap();
        assert!(prog.programs[0].has_configuration_steps());
    }
    #[test]
    fn test_has_install() {
        let prog = from_json_file("example.json".to_owned()).unwrap();
        assert!(prog.programs[0].has_installation_steps());
    }

    #[test]
    fn test_has_dependencies() {
        // this test is kinda trash but i'm too tired to make it good
        let prog = from_json_file("example.json".to_owned()).unwrap();
        assert!(prog.programs[0].has_dependencies() == false);
    }

}
