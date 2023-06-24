//! # ginst
//!
//! ginst is a command line utility used to read and execute installation and configuration
//! commands from specified in a JSON file.
//!
//! To get started download and execute the app:
//!
//! ```bash
//! cargo install ginst
//! ginst --file /path/to/file.json
//! ```
//!
//! For more information see the [ginst Wiki](https://github.com/Sebbito/ginst/wiki)

pub mod commands;

// use libginst::types::FileType;
use libginst::{
    types::{Programable, ExecutionError},
    program,
    program::Program,
    executor,
    parser,
    package_manager::get_package_manager
};
use commands::Command;
use clap::Parser;
use std::env;

/// Args struct holding the CL args
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    /// execute quick operations on programs and exit
    #[command(subcommand)]
    command: Option<Command>,

    /// Path to the file holding program information
    file: String,

    /// count all programs (including dependencies)
    #[arg(long, group = "cli")]
    count: bool,

    /// count all missing programs (including dependencies)
    #[arg(long, group = "cli")]
    count_missing: bool,

    /// perform checks if file is syntax is ok
    #[arg(long, group = "cli")]
    check: bool,

    /// list all the programs contained in the file
    #[arg(long, group = "cli")]
    list: bool,

    /// list programs and check if they are installed
    #[arg(long, group = "cli")]
    status: bool,

    /// The shell in which the command shall be executed
    #[arg(value_enum)]
    shell: Option<String>,
}

fn main() -> Result<(), ExecutionError> {
    // output debug info if the build is a debug build
    if cfg!(debug_assertions) {
        env::set_var("RUST_BACKTRACE", "1");
    }
    // Parse arguments
    let args = Arguments::parse();

    // Check if we can spawn processes with user suggested shell or just use `sh`
    let shell = executor::eval_shell(args.shell);
    // set this as a global variable via environment variables so that future executors can access this easier
    // TODO: find a better way to use these "dynamic global variables"
    // TODO: OMFG FIX THIS ASAP
    env::set_var("EXECUTE_SHELL", shell);

    // open file and parse the file contents
    let file = &args.file;
    let programs: Vec<Program> = parser::get_programs_from_file(file);

    if args.count {
        println!("{}", program::count(&programs));
    } else if args.count_missing {
        println!("{}", program::count_missing(&programs));
    } else if args.check {
        // parser already ran and succeeded
        println!("File looks good!");
    } else if args.list {
        program::print_name(&programs);
    } else if args.status {
        program::print_status(&programs);
    } else if let Some(command) = args.command {
        match &command {
            Command::Install { use_package_manager, program } => {
                if let Some(program_name) = program {
                    // user only wants to install one certain program
                    if let Some(prog) = program::search_from_name(program_name, &programs){
                        install_with_pm_check(prog, *use_package_manager)?;
                    } else {
                        println!("No program with name {} found", program_name);
                    }
                } else {
                    for p in programs.iter() {
                        install_with_pm_check(p, *use_package_manager)?;
                    }
                }
            }
            Command::Configure { all, program } => {
                if *all {
                    program::configure_all(&programs)?;
                } else if let Some(program_name) = program {
                    // user only wants to configure one certain program
                    if let Some(prog) = program::search_from_name(program_name, &programs){
                        prog.configure()?;
                    } else {
                        println!("No program with name {} found", program_name);
                    }
                } else {
                    panic!("No option on configure");
                }
            }
            // Command::Export { filetype } => match filetype {
            //     FileType::Json => {
            //         let string = serde_json::to_string_pretty(&programs).unwrap();
            //         let new_file = Path::new(file).with_extension("json");
            //         std::fs::write(new_file, string).unwrap();
            //     }
            //     FileType::Yaml => {
            //         let string = serde_yaml::to_string(&programs).unwrap();
            //         let new_file = Path::new(file).with_extension("yml");
            //         std::fs::write(new_file, string).unwrap();
            //     }
            // },
        }
    } else {
        // user didn't use any known command
        println!("Please specify what to do.");
        println!("See `ginst --help` for help.");
    }
    Ok(())
}

// TODO: maybe not put this here...this is just a quick hacky way to not
// get a headache
fn install_with_pm_check(p: &Program, use_pm: bool) -> Result<(), ExecutionError> {
    // try installing manually first
    if let Err(error) = p.install() {
        // if the installation failed we're fucked
        if matches!(error, ExecutionError::InstallError) {
            return Err(ExecutionError::InstallError);
        } else {
            // the remaining error cases are that the instruction set is empty
            // for this case, if the user requested to use the package manager
            // we will do so
            if use_pm{
                if let Some(pm) = get_package_manager() {
                    if pm.install(p.get_name().as_ref()).is_ok() {
                        return Ok(());
                    }
                    dbg!(pm);
                    println!("Could not install program {}", p.get_name());
                } else {
                    println!("Package manager not found!");
                }
            }
            return Err(ExecutionError::InstallError);
        }
    }
    Ok(())
}
