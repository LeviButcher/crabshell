use nix::unistd::{execvp, fork, ForkResult};
use std::error::Error;
use std::ffi::{CStr, CString};
use std::io::{stdin, BufRead};

struct Command_Call {
    command_name: CString,
    args: Vec<CString>,
}

impl<'a> Command_Call {
    // TODO: make better split function for args
    fn new(input: &'a str) -> Command_Call {
        let split_input: Vec<&str> = input.split(' ').collect();
        let (command_name, args) = match split_input.split_first() {
            Some((a, b)) => (a, b),
            None => panic!("Levi you should of handled the error here"),
        };
        let command_name = CString::new(command_name.as_bytes()).unwrap();
        let args: Vec<CString> = args
            .iter()
            .map(|x| CString::new(x.as_bytes()).unwrap())
            .collect();

        Command_Call { command_name, args }
    }

    fn execute(&self) -> Result<String, String> {
        match fork() {
            Ok(ForkResult::Parent { child }) => Ok("Started Command".into()),
            Ok(ForkResult::Child) => {
                let mut args: Vec<&CStr> = Vec::new();
                for arg in &self.args {
                    args.push(arg.as_c_str());
                }

                match execvp(&self.command_name[..], &args[..]) {
                    Err(_) => Err("Sub Process Failed".into()),
                    _ => Ok("Command was ran".into()),
                }
            }
            Err(_) => Err("Process starting failed".into()),
        }
    }
}

/// Starts up Terminal process
/// Loops until Error
pub fn run() {
    let mut stdio = stdin();
    let user_input = read_user_input(&mut stdio.lock()).unwrap();
    let command = Command_Call::new(&user_input);
    command.execute();
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
    fn command_call_new_should_return_back_command_call() {
        let user_input = "ls -la";
        let command = super::Command_Call::new(&user_input);
        let expected_name = CString::new("ls").expect("CString::new failed");
        let args = vec![CString::new("-la").expect("CString::new failed")];
        assert_eq!(command.command_name, expected_name);
        assert_eq!(command.args, args);
    }
}
