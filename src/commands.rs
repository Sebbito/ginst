
/// # Commands
/// 
/// Subcommands and their arguments
use clap::Subcommand;
// use libginst::types::FileType;

#[derive(Subcommand)]
pub enum Command {
    /// Execute installation steps
    Install {
        /// use the local package manager?
        #[arg(long, short)]
        use_package_manager: bool,

        /// the Program to install
        program: Option<String>,

    },

    /// Execute configuration steps
    Configure {
        #[arg(long, short)]
        /// configure all
        all: bool,

        /// the Program to configure
        program: Option<String>,

    },

    // Export the given file to a different format
    // Export {
    //     #[arg(value_enum)]
    //     filetype: FileType,
    // },
}
