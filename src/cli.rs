use clap::{Subcommand, ValueEnum};

#[derive(Subcommand)]
pub enum Command {
    Install {
        #[arg(long, short)]
        /// install all (missing)
        all: bool,
    },

    Configure {
        #[arg(long, short)]
        /// configure all
        all: bool,
    },

    Export {
        #[arg(value_enum)]
        filetype: FileType
    },

    List {
        #[arg(long)]
        status: bool
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum FileType {
    Json,
    Yaml,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
}
