use crate::program;
use json::JsonValue::{self, Null};

/*
* This package provides utility functions for the Program Struct like creating a Vector of
* programs, building the depenency list and so on.
*/

pub fn build_dependency_list(dependencies: JsonValue) -> Vec<program::Program> {
    if dependencies != Null {
        return program::as_vec_from_json(dependencies);
    } else {
        vec![]
    }
}

pub fn install_missing(programs: Vec<program::Program>) {
    for prog in programs {
        prog.install();
    }
}

pub fn count_missing(programs: Vec<program::Program>) -> u8 {
    let mut counter = 0;
    for p in programs {
        if !p.is_installed() {
            counter += 1;
        }
    }
    counter
}

