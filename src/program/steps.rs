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

            if !output.status.success() {
                if cfg!(debug_assertions) {
                    println!("{0:#?}", output.clone());
                }
                println!("{}", String::from_utf8(output.stderr).unwrap());
                panic!("Installation instruction didn't finish correctly. Aborting")
            }
        }
    }

    pub fn len(&self) -> usize {
        self.steps.len()
    }
}
