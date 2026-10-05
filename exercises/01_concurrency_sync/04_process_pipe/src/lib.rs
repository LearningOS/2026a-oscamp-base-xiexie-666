use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

pub fn run_command(program: &str, args: &[&str]) -> String {
    let output = Command::new(program).args(args).stdout(Stdio::piped()).output().unwrap();
    String::from_utf8(output.stdout).unwrap()
}

pub fn pipe_through_cat(input: &str) -> String {
    let mut child = Command::new("cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    {
        let mut stdin = child.stdin.take().unwrap();
        stdin.write_all(input.as_bytes()).unwrap();
    }
    let mut output = String::new();
    child.stdout.take().unwrap().read_to_string(&mut output).unwrap();
    child.wait().unwrap();
    output
}

pub fn get_exit_code(command: &str) -> i32 {
    Command::new("sh")
        .args(["-c", command])
        .status()
        .unwrap()
        .code()
        .unwrap_or(-1)
}

pub fn run_command_with_result(program: &str, args: &[&str]) -> io::Result<String> {
    let output = Command::new(program).args(args).stdout(Stdio::piped()).output()?;
    String::from_utf8(output.stdout)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

pub fn pipe_through_grep(pattern: &str, input: &str) -> String {
    let mut child = Command::new("grep")
        .arg(pattern)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    {
        let mut stdin = child.stdin.take().unwrap();
        stdin.write_all(input.as_bytes()).unwrap();
    }
    let mut output = String::new();
    child.stdout.take().unwrap().read_to_string(&mut output).unwrap();
    child.wait().unwrap();
    output
}
