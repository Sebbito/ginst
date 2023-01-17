use crate::program::Program;
use json::JsonValue::{self, Null};
use crate::program::steps::Steps;
use crate::program::steps;

/*
* This package provides utility functions for the Program Struct like creating a Vector of
* programs, building the depenency list and so on.
*/

pub fn build_dependency_list(dependencies: JsonValue) -> Vec<Program> {
    if dependencies != Null {
        return as_vec_from_json(dependencies);
    } else {
        vec![]
    }
}

pub fn from_json(json_parsed: &JsonValue) -> Program {
    let mut prog: Program = Default::default();
    let mut install_vec: Vec<Steps> = vec![];

    let s_vec = steps::from_json(json_parsed["install"].clone());
    install_vec.append(&mut s_vec.clone());
    
    prog.name = json_parsed["name"].clone().to_string();
    prog.install = install_vec;
    prog.status = prog.check();

    if json_parsed["dependencies"] != Null {
        prog.dependencies = build_dependency_list(json_parsed["dependencies"].clone());
    } else {
        prog.dependencies = vec![];
    }
    
    prog
}

pub fn as_vec_from_json(json_parsed: JsonValue) -> Vec<Program>{
    let mut programs: Vec<Program> = vec![];

    for program in json_parsed["programs"].members() {
        programs.push(from_json(program));
    }
    
    return programs;
}

pub fn install_missing(programs: Vec<Program>) {
    for prog in programs {
        prog.install();
    }
}

pub fn count_missing(programs: Vec<Program>) -> u8 {
    let mut counter = 0;
    for p in programs {
        if !p.is_installed() {
            counter += 1;
        }
    }
    counter
}

