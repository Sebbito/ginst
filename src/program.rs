use std::process::Command;
use crate::program::steps::Steps;
use fti::get_dist;

pub mod util;
pub mod display;
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
    install: Vec<Steps>,
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
        let os = get_dist();
        if self.status == Status::Missing {
            if self.install.len() != 0 {
                for instruction in self.install.clone() {
                    match instruction.clone().dists {
                        os => {
                            println!("Installing programm {}...", self.name);
                            instruction.execute();
                        }
                    }
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
        for dep in self.dependencies.clone() {
            print!("  "); // indent by 1 block
            dep.print_status();
        }
    }
}
