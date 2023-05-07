use std::{
    env,
    process::{Command, ExitStatus, Stdio},
};

#[derive(Debug, Clone)]
pub struct Executor {
    shell: String,
}

impl Executor {
    /// Returns a new Executor. Uses `sh` as default shell unless `EXECUTE_SHELL` is set in the
    /// environment variables.
    pub fn new() -> Executor {
        // Set the shell to the environment variable specified. Defaults to `sh`.
        let shell = match env::var("EXECUTE_SHELL") {
            Ok(shell) => shell,
            Err(_) => "sh".to_owned(),
        };
        Executor { shell }
    }

    /// Executes each step in `steps`.
    pub fn execute_steps(&self, steps: &Vec<String>) {
        for step in steps.iter() {
            self.execute(step)
                .expect("Something went wrong on execution");
        }
    }

    /// Executes single command `command` in a shell.
    pub fn execute(&self, command: &String) -> std::io::Result<ExitStatus> {
        Command::new(self.shell.clone())
            .arg("-c")
            .arg(command.clone())
            .stdout(Stdio::null())
            .status()
    }
}
/// Evaluates if the given shell is on the system and executable. Returns 'sh' per default.
pub fn eval_shell(opt_shell: Option<String>) -> String {
    if let Some(shell) = opt_shell.clone() {
        // yes it tests type...only made it this way so that the process quits
        let status = Command::new(shell.clone())
            .arg("-c")
            .arg("type type")
            .stdout(Stdio::null())
            .status()
            .expect("Could not execute shell");

        match status.success() {
            true => shell,
            false => "sh".to_owned(),
        }
    } else {
        "sh".to_owned()
    }
}
