use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{chdir, execvp, fork, ForkResult};
use std::error::Error;
use std::ffi::{CStr, CString};
use std::io::{stdin, BufRead};

fn cs_cd(args: &Vec<CString>) {
    chdir(args[1].as_c_str()).unwrap();
}
fn cs_exit(args: &Vec<CString>) {}
fn cs_help(args: &Vec<CString>) {
    println!("Levi Butcher's CrabShell");
    println!("CrabPeople CrabPeople, Taste Like Crab, Talk like People");
}

struct CommandCall {
    command_name: CString,
    args: Vec<CString>,
}

impl<'a> CommandCall {
    fn new(input: &'a str) -> CommandCall {
        let split_input: Vec<&str> = input
            .split(|x| x == ' ' || x == '\n')
            .filter(|x| x != &"")
            .collect();
        let command_name = split_input.first().unwrap();
        let command_name = CString::new(command_name.as_bytes()).unwrap();
        let args: Vec<CString> = split_input
            .iter()
            .map(|x| CString::new(x.as_bytes()).unwrap())
            .collect();

        CommandCall { command_name, args }
    }

    fn execute(&self) -> Result<String, String> {
        if self.command_name == CString::new("cd".as_bytes()).unwrap() {
            cs_cd(&self.args);
            return Ok("Good to go".into());
        }
        // Doesn't actually work yet, need to refactor to send back a code to quit loop
        if self.command_name == CString::new("exit".as_bytes()).unwrap() {
            cs_exit(&self.args);
            return Ok("Good to go".into());
        }
        if self.command_name == CString::new("help".as_bytes()).unwrap() {
            cs_help(&self.args);
            return Ok("Good to go".into());
        }

        match fork() {
            Ok(ForkResult::Parent { child }) => loop {
                match waitpid(child, None) {
                    Ok(WaitStatus::Exited(_, _)) => return Ok("Process completed".into()),
                    Ok(_) => continue,
                    Err(_) => return Err("Child process failed".into()),
                };
            },
            Ok(ForkResult::Child) => {
                let mut args: Vec<&CStr> = Vec::new();
                for arg in &self.args {
                    args.push(arg.as_c_str());
                }
                println!("{:?}", args);

                match execvp(&self.command_name[..], &args[..]) {
                    Err(_) => Err("Sub Process Failed".into()),
                    _ => Ok("Command was ran".into()),
                }
            }
            Err(_) => Err("Process starting failed".into()),
        }
    }
}

pub fn run() {
    let stdio = stdin();
    let user_input = read_user_input(&mut stdio.lock()).unwrap();
    let command = CommandCall::new(&user_input);

    if let Err(error) = command.execute() {
        println!("{}", error)
    }
}

pub fn read_user_input<T: BufRead>(input_reader: &mut T) -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    input_reader.read_line(&mut input)?;
    Ok(input)
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use std::io::BufReader;

    #[test]
    fn read_user_input_should_return_back_string_vector() {
        let input = "I am input".as_bytes();
        let mut reader = BufReader::new(input);
        let result = super::read_user_input(&mut reader).unwrap();
        assert_eq!(result, "I am input");
    }

    #[test]
    fn commandcall_new_should_return_back_commandcall() {
        let user_input = "ls -la";
        let command = super::CommandCall::new(&user_input);
        let expected_name = CString::new("ls").expect("CString::new failed");
        let args = vec![
            CString::new("ls").expect("CString::new failed"),
            CString::new("-la").expect("CString::new failed"),
        ];
        assert_eq!(command.command_name, expected_name);
        assert_eq!(command.args, args);
    }
}
