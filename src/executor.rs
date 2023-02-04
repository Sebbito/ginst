use crate::cli::Shell;
use std::process::{Command, ExitStatus};

#[derive(Debug)]
pub struct Executor {
    shell: Option<Shell>,
    command: String,
}

impl Executor {
    pub fn new(shell: Option<Shell>, command: String) -> Executor {
        Executor { shell, command }
    }

    pub fn execute(&self) -> std::io::Result<ExitStatus> {
        Command::new(self.shell_as_string()).arg("-c").arg(self.command.clone()).status()
    }

    fn shell_as_string(&self) -> String {
        match self.shell {
            Some(Shell::Zsh) => "zsh".to_owned(),
            Some(Shell::Fish) => "fish".to_owned(),
            Some(Shell::Bash) => "bash".to_owned(),
            None => "bash".to_owned(), // fall back to bash per default
        }
    }
}
