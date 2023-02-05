use crate::{Arguments, display};
use crate::{program, parser, types::{Command, FileType}};
use std::{path::Path, error::Error};

pub fn handle_arguments(args: Arguments) -> Result<(), Box<dyn Error>> {
    let file = &args.file;
    let programs: Vec<program::Program>= parser::get_programs_from_file(&args.file);

    if args.count {
        println!("{}", program::count(&programs));
    } else if args.count_missing {
        println!("{}", program::count_missing(&programs));
    } else if args.check {
        // parser already ran
        println!("File looks good!");
    } else if let Some(command) = args.command {
        match &command {
            Command::Install { all } => {
                if *all {
                    program::install_missing(&programs);
                }
            },
            Command::Configure { all } => {
                if *all {
                    program::configure_all(&programs);
                }
            },
            Command::Export { filetype } => {
                match filetype {
                    FileType::Json => {
                        let string = serde_json::to_string_pretty(&programs).unwrap();
                        let new_file = Path::new(&file).with_extension("json");
                        std::fs::write(new_file, string).unwrap();
                    },
                    FileType::Yaml => {
                        let string = serde_yaml::to_string(&programs).unwrap();
                        let new_file = Path::new(&file).with_extension("json");
                        std::fs::write(new_file, string).unwrap();

                    },
                }
            },
            Command::List { status } => {
                if *status {
                    program::print_status(&programs);
                } else {
                    program::print_name(&programs);
                }
            }
        }
    } else {
        let app = display::tui::App::new(programs.clone());
        if let Err(error) = display::run_app(&app) {
            panic!("{:?}", error);
        }
    }
    Ok(())

}
