use crate::ShellOutput;

pub fn run(args: Vec<String>) -> ShellOutput {
    ShellOutput {
        stdout: Some(args.join(" ")),
        stderr: None
    }
}
