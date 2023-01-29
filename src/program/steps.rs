//! # Steps
//!
//! Crate containing structs representing execution steps


use std::process::Command;
use serde::{Serialize, Deserialize};

/// Steps struct representing execution steps
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Steps {
    /// Multiple steps can be for multiple distributions so the dists attribute is also a vec
    pub distro: Vec<String>,
    pub steps: Vec<String>
}

impl Steps {
    pub fn execute(&self) {
        for step in self.steps.clone() {

            if cfg!(debug_assertions) {
                println!("{0:#?}", step.clone());
            }

            let output = Command::new("bash").arg("-c").arg(step).output().expect("Could not execute command");

            println!("{}", String::from_utf8(output.stdout).unwrap());

            if !output.status.success() {
                println!("{}", String::from_utf8(output.stderr).unwrap());
                panic!("Instruction didn't finish correctly. Aborting")
            }
        }
    }

    pub fn len(&self) -> usize {
        self.steps.len()
    }

    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
}


/// Struct representing a collection of steps
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstructionSet{
    execution_steps: Vec<Steps>
}

impl InstructionSet {

    pub fn len(&self) -> usize {
        self.execution_steps.len()
    }

    pub fn is_empty(&self) -> bool {
        self.execution_steps.is_empty()
    }

    pub fn push(&mut self, step: Steps) {
        self.execution_steps.push(step);
    }
}
