use crate::program::steps::Steps;
use json::JsonValue;

#[derive(Debug, Clone, Default)]
pub struct InstructionSet{
    execution_steps: Vec<Steps>
}

impl InstructionSet {
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

pub fn from_json(json_parsed: JsonValue) -> InstructionSet{
    let mut set: InstructionSet = Default::default();

    for (raw_dist, raw_steps) in json_parsed.entries() {
        let dists = raw_dist.split(',').map(|s| s.to_string()).collect();
        let steps = raw_steps.members().map(|m| m.to_string()).collect();

        set.execution_steps.push(Steps { dists, steps });
    }

    set
}

