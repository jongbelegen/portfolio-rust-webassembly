use crate::ShellOutput;

pub fn run() -> ShellOutput {
    ShellOutput {
        stdout: Some(String::from("\u{001b}c")),
        stderr: None,
    }
}
