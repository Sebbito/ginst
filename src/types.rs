use std::error::Error;

/// # Types
///
/// Different types used throughout the program
use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum FileType {
    Json,
    Yaml,
}

/// Trait that promises to have a run function.
pub trait Runnable {
    fn run(&self) -> Result<(), Box<dyn Error>>;
}

/// Trait that promises to return a sublist of items with the same type.
pub trait Sublistable<T = Self> {
    fn get_sublist(&self) -> Vec<T>;
}

/// Trait that defines basic functionality.
pub trait Programable: Sublistable + Clone {
    fn get_name(&self) -> String;
    fn install(&self);
    fn configure(&self);
    fn is_installed(&self) -> bool;
}

/// Trait that defines an execute() function.
pub trait Executable {
    fn execute(&self);
}
