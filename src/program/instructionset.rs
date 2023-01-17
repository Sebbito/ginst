use crate::program::steps::Steps;
use json::JsonValue;

#[derive(Debug, Clone, Default)]
pub struct InstructionSet{
    steps: Vec<Steps>
}

impl InstructionSet {
    pub fn for_dist(&self, dist_name: String) -> Option<Steps> {
        for steps in self.steps.clone() {
            for dist in steps.dists.clone() {
                if dist == dist_name || dist == "*"{
                    return Some(steps)
                }
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.steps.len()
    }

    pub fn push(&mut self, step: Steps) {
        self.steps.push(step);
    }
}

pub fn from_json(json_parsed: JsonValue) -> InstructionSet{
    let mut set: InstructionSet = Default::default();

    for (raw_dist, raw_steps) in json_parsed.clone().entries() {
        let dists = raw_dist.clone().split(",").map(|s| s.to_string()).collect();
        let steps = raw_steps.clone().members().map(|m| m.to_string()).collect();

        set.steps.push(Steps { dists, steps });
    }

    set
}
