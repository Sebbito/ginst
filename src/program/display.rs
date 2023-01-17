use crate::program::Program;

pub fn print_all(programs: Vec<Program>) {
    for prog in programs {
        prog.print_all();
    }
}

