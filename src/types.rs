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

/// promises to have a run function
pub trait Runnable {
    fn run(&self) -> Result<(), Box<dyn Error>>;
}

/// promises to return a sublist of items with the same type
pub trait Sublistable<T = Self> {
    fn get_sublist(&self) -> Vec<T>;
}

pub trait Programable: Sublistable + Clone {
    fn get_name(&self) -> String;
    fn install(&self);
    fn configure(&self);
    fn is_installed(&self) -> bool;
}
pub trait Executable {
    fn execute(&self);
}
