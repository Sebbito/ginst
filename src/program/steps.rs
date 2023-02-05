//! # Steps
//!
//! Crate containing structs representing execution steps


use serde::{Serialize, Deserialize};

use crate::{types::Shell, executor::Executor};

/// Steps struct representing execution steps
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Steps {
    /// Multiple steps can be for multiple distributions so the dists attribute is also a vec
    pub distro: Vec<String>,
    pub steps: Vec<String>
}

impl Steps {
    pub fn execute(&self, shell: &Option<Shell>) {
        for step in self.steps.clone() {
            let result = Executor::new(shell.clone(), step).execute();
            if let Err(error) = result {
                panic!("{}", error);
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

/// utility function to get instructions for currect dist
pub fn steps_for_dist<'a>(list: &'a Vec<Steps>, distro: &String) -> Option<&'a Steps> {
    for steps in list {
        for dist in steps.distro.clone() {
            if dist.eq(distro) || dist == "*"{
                return Some(steps)
            }
        }
    }
    None
}
