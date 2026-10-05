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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_echo() {
        let output = run_command("echo", &["hello"]);
        assert_eq!(output.trim(), "hello");
    }

    #[test]
    fn test_run_with_args() {
        let output = run_command("echo", &["-n", "no newline"]);
        assert_eq!(output, "no newline");
    }

    #[test]
    fn test_pipe_cat() {
        let output = pipe_through_cat("hello pipe!");
        assert_eq!(output, "hello pipe!");
    }

    #[test]
    fn test_pipe_multiline() {
        let input = "line1\nline2\nline3";
        assert_eq!(pipe_through_cat(input), input);
    }

    #[test]
    fn test_exit_code_success() {
        assert_eq!(get_exit_code("true"), 0);
    }

    #[test]
    fn test_exit_code_failure() {
        assert_eq!(get_exit_code("false"), 1);
    }

    #[test]
    fn test_run_command_with_result_success() {
        let result = run_command_with_result("echo", &["hello"]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().trim(), "hello");
    }

    #[test]
    fn test_run_command_with_result_nonexistent() {
        let result = run_command_with_result("nonexistent_command_xyz", &[]);
        // Should be an error because command not found
        assert!(result.is_err());
    }

    #[test]
    fn test_pipe_through_grep_basic() {
        let input = "apple\nbanana\ncherry\n";
        let output = pipe_through_grep("a", input);
        // grep outputs matching lines with newline
        assert_eq!(output, "apple\nbanana\n");
    }

    #[test]
    fn test_pipe_through_grep_no_match() {
        let input = "apple\nbanana\ncherry\n";
        let output = pipe_through_grep("z", input);
        // No lines match -> empty string
        assert_eq!(output, "");
    }

    #[test]
    fn test_pipe_through_grep_multiline() {
        let input = "first line\nsecond line\nthird line\n";
        let output = pipe_through_grep("second", input);
        assert_eq!(output, "second line\n");
    }
}
