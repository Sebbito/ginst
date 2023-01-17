use std::process::Command;
use json::JsonValue;

#[derive(Debug, Default, Clone)]
pub struct Steps {
    pub dist: String,
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

    for (dist, steps) in json_parsed.clone().entries() {
        let mut s: Vec<String> = vec![];
        for step in steps["steps"].members() {
            s.push(step.to_string());
        }
        steps_vec.push(Steps { dist: dist.to_string(), steps: s });
    }

    steps_vec
}
