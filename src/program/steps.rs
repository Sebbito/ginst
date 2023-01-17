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
        for cmd in self.steps.clone() {
            let step: Vec<&str> = cmd.split(" ").collect();
            let function = step[0].clone();
            let args: Vec<&str>=step.into_iter().skip(1).collect();
            let status = Command::new(function).args(args).status().expect("Could not execute command");

            if !status.success() {
                panic!("Installation instruction didn't finish correctly. Aborting")
            }
        }
    }

    pub fn len(&self) -> usize {
        self.steps.len()
    }
}
