use json::JsonValue;
use crate::program::steps::Steps;

#[derive(Debug, Clone, Default)]
pub struct Configuration {
    steps: Vec<Steps>
}

impl Configuration {
    pub fn for_dist(&self, dist_name: String) -> Option<Steps> {
        for configuration in self.steps.clone() {
            for dist in configuration.dists.clone() {
                if dist == dist_name || dist == "*"{
                    return Some(configuration)
                }
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.steps.len()
    }
}

pub fn from_json(json_parsed: JsonValue) -> Configuration {
    let mut configuration: Configuration = Default::default();

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
        configuration.steps.push(Steps { dists, steps });
    }

    configuration
}
