use nix::unistd::{execvp, fork, ForkResult};
use std::ffi::{CStr, CString};

fn main() {
    println!("Hello, world!");

    match fork() {
        Ok(ForkResult::Parent { child }) => {
            println!("Parent Process, childpid is {}", child);
        }
        Ok(ForkResult::Child) => {
            println!("Child Process started");

            let commandName = CString::new("ls").expect("error");
            let temp = CString::new("-l").expect("error");
            let args: [&CStr; 1] = [&temp];
            match execvp(commandName.as_c_str(), &args) {
                Err(_) => println!("Big ooff"),
                _ => println!("It worked!"),
            }
        }
        Err(_) => println!("Process starting failed"),
    }
}

// accept user input
// parse user input into command name / arguments
// fork a process and wait for completion?
