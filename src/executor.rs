use std::process::{Command, ExitStatus, Stdio};

#[derive(Debug)]
pub struct Executor {
    shell: String,
    command: String,
}

impl Executor {
    pub fn new(shell: Option<String>, command: String) -> Executor {
        let shell = Executor::eval_shell(shell);
        Executor { shell, command }
    }

    pub fn execute(&self) -> std::io::Result<ExitStatus> {
        Command::new(self.shell.clone())
            .arg("-c")
            .arg(self.command.clone())
            .stdout(Stdio::null())
            .status()
    }

    /// Evaluates if the given shell is on the system and executable. Returns 'sh' per default
    fn eval_shell(opt_shell: Option<String>) -> String {
        if let Some(shell) = opt_shell.clone() {
            let status = Command::new(shell.clone())
                // .stdout(Stdio::null())
                .status()
                .unwrap();

            match status.success() {
                true => shell,
                false => "sh".to_owned()
            }
        } else {
            "sh".to_owned()
        }
    }
}
