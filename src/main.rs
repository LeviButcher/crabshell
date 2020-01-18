use crabshell::run;
use std::io;
use std::io::prelude::*;

fn main() {
    println!("Starting up CrabShell <-c->");
    loop {
        print!("\nCS:>");
        io::stdout().flush().unwrap();
        run();
    }
}
