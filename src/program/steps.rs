use std::process::Command;
use json::JsonValue;

/*
 * Base Steps struct with implementations for functionality.
 */

#[derive(Debug, Default, Clone)]
pub struct Steps {
    pub dists: Vec<String>, // instructions can be for multiple distributions
    steps: Vec<String>
}

impl Steps {
    pub fn execute(&self) {
        let mut output;

        for cmd in self.steps.clone() {
            let step: Vec<&str> = cmd.split(" ").collect();
            let function = step[0].clone();
            let args: Vec<&str>=step.into_iter().skip(1).collect();
            output = Command::new(function).args(args).output().expect("Could not execute command");

            if !output.status.success() {
                panic!("Installation instruction didn't finish correctly. Aborting")
            }
        }
    }

    pub fn size(&self) -> usize {
        self.steps.len()
    }
}

pub fn from_json(json_parsed: JsonValue) -> Vec<Steps> {
    let mut steps_vec: Vec<Steps> = vec![];

    for (raw_dist, raw_steps) in json_parsed.clone().entries() {
        let mut dists: Vec<String> = vec![];
        let splits = raw_dist.clone().split(",");

        for split in splits {
            dists.push(split.to_string());
        }

        let mut steps: Vec<String> = vec![];
        for step in raw_steps["steps"].members() {
            steps.push(step.to_string());
        }
        steps_vec.push(Steps { dists, steps });
    }

    steps_vec
}
