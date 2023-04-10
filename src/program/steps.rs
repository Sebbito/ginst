//! # Steps
//!
//! Crate containing structs representing execution steps

use crate::executor::Executor;
use serde::{Deserialize, Serialize};

/// Steps struct representing execution steps
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Steps {
    /// Multiple steps can be for multiple distributions so the dists attribute is also a vec
    pub distro: Vec<String>,
    pub steps: Vec<String>,
}

impl Steps {
    pub fn execute(&self) {
        Executor::new().execute_steps(self.steps.clone());
    }

    pub fn len(&self) -> usize {
        self.steps.len()
    }

    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
}

/// utility function to get instructions for currect dist
pub fn steps_for_dist(list: Vec<Steps>, distro: &String) -> Option<Steps> {
    for steps in list {
        for dist in steps.distro.clone() {
            if dist.eq(distro) || dist == "*" {
                return Some(steps);
            }
        }
    }
    None
}
