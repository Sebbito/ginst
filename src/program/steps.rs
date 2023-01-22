use std::process::Command;

/*
 * Base Steps struct with implementations for functionality.
 */

#[derive(Debug, Default, Clone)]
pub struct Steps {
    pub dists: Vec<String>, // instructions can be for multiple distributions
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
