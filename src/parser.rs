use crate::program::Program;
use serde_yaml;
use serde_json;

fn from_json_file(file_contents: &String) -> Vec<Program> {
    let result = serde_json::from_str::<Vec<Program>>(file_contents);
    match result {
        Ok(programs) => programs,
        Err(error) => panic!("{:?}", error)
    }
}

fn from_yaml_file(file_contents: &String) -> Vec<Program> {
    let result = serde_yaml::from_str::<Vec<Program>>(file_contents);
    match result {
        Ok(programs) => programs,
        Err(error) => panic!("{:?}", error)
    }
}

pub fn get_programs_from_file(path: &str) -> Vec<Program> {
    let binding = std::path::Path::new(&path).canonicalize().expect("Could not unfold given Path");
    let extension = binding.extension().unwrap();
    let file_contents = std::fs::read_to_string(&path).unwrap();

    let mut programs = match extension.to_str().unwrap() {
        "json" => from_json_file(&file_contents),
        "yaml" | "yml" => from_yaml_file(&file_contents),
        _ => panic!("Unsupported file type {}", extension.to_str().unwrap())
    };

    for program in programs.iter_mut().by_ref() {
        program.set_status();
    }

    programs
}
