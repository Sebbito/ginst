use crate::program::Program;

pub fn print_all(programs: Vec<Program>) {
    let mut counter = 1;
    for prog in programs {
        print!("{}: ", counter);
        prog.print();
        counter += 1;
    }
}

pub fn print_top_level(programs: Vec<Program>) {
    let mut counter = 1;

    for prog in programs {
        print!("{}: ", counter);
        prog.print_status();
        counter += 1;
    }
}
