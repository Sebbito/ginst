
/// # Commands
/// 
/// Subcommands and their arguments
use clap::Subcommand;
use crate::types::FileType;

#[derive(Subcommand)]
pub enum Command {
    /// Execute installation steps
    Install {
        #[arg(long, short)]
        /// install all (missing)
        all: bool,
    },

    /// Execute configuration steps
    Configure {
        #[arg(long, short)]
        /// configure all
        all: bool,
    },

    /// Export the given file to a different format
    Export {
        #[arg(value_enum)]
        filetype: FileType,
    },
}
