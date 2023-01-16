use std::process::Command;

pub fn get_dist() -> String {
    let output = Command::new("grep")
                    .args(["^NAME=\".*\"","/etc/os-release"])
                    .output()
                    .expect("Failed to get os information");
    let raw = String::from_utf8(output.stdout).expect("Failed to generate string from stoud");

    return raw.replace("\"", "").replace("\n", "").chars().skip(5).collect();
}
