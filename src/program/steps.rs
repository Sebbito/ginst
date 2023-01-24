//! # Steps
//!
//! Crate containing structs representing execution steps


use std::process::Command;
use json::JsonValue;

/// Steps struct representing execution steps
#[derive(Debug, Default, Clone)]
pub struct Steps {
    /// Multiple steps can be for multiple distributions so the dists attribute is also a vec
    pub dists: Vec<String>,
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
#[derive(Debug, Clone, Default)]
pub struct InstructionSet{
    execution_steps: Vec<Steps>
}

impl InstructionSet {
    /// Utilitiy function returning a Steps struct for the users distribution
    pub fn for_dist(&self, dist_name: String) -> Option<Steps> {
        for steps in self.execution_steps.clone() {
            for dist in steps.dists.clone() {
                if dist == dist_name || dist == "*"{
                    return Some(steps)
                }
            }
        }
        None
    }

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

/// Generates an Instructionsset from parsed json
pub fn from_json(json_parsed: JsonValue) -> InstructionSet{
    let mut set: InstructionSet = Default::default();

    for (raw_dist, raw_steps) in json_parsed.entries() {
        let dists = raw_dist.split(',').map(|s| s.to_string()).collect();
        let steps = raw_steps.members().map(|m| m.to_string()).collect();

        set.execution_steps.push(Steps { dists, steps });
    }

    set
}

