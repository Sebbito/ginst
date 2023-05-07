use crate::program::Program;
use serde_json;
use serde_yaml;

/// Serde call to parse json string `file_contents` to a Vec of `Program`'s.
fn from_json_file(file_contents: &String) -> Vec<Program> {
    let result = serde_json::from_str::<Vec<Program>>(file_contents);
    match result {
        Ok(programs) => programs,
        Err(error) => panic!("{:?}", error),
    }
}

/// Serde call to parse yaml string `file_contents` to a Vec of `Program`'s.
fn from_yaml_file(file_contents: &String) -> Vec<Program> {
    let result = serde_yaml::from_str::<Vec<Program>>(file_contents);
    match result {
        Ok(programs) => programs,
        Err(error) => panic!("{:?}", error),
    }
}

/// Function that reads file contents from `path` and parses them into a Vec of `Program`'s.
/// Also sets the programs status (Installed/Missing).
pub fn get_programs_from_file(path: &str) -> Vec<Program> {
    // Extract the file extension.
    let binding = std::path::Path::new(&path)
        .canonicalize()
        .expect("Could not unfold given Path");
    let extension = binding.extension().unwrap();
    
    // Read file contents.
    let file_contents = std::fs::read_to_string(&path).unwrap();

    // Depending on file extension, call different parser function.
    let mut programs = match extension.to_str().unwrap() {
        "json" => from_json_file(&file_contents),
        "yaml" | "yml" => from_yaml_file(&file_contents),
        _ => panic!("Unsupported file type {}", extension.to_str().unwrap()),
    };

    // Set program status.
    for program in programs.iter_mut().by_ref() {
        program.set_status();
    }

    programs
}
