#[derive(Default, Debug)]
pub struct ShellState {
    pub current_dir: String,
    pub output: ShellOutput,
}
#[derive(Default, Debug)]
pub struct ShellOutput {
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

impl ShellOutput {
    pub fn set_stdout(&mut self, stdout: String) {
        self.stdout = Some(stdout.clone());
        self.stderr = None;
    }

    pub fn clear(&mut self) {
        self.stdout = None;
        self.stderr = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_output_set_stdout() {
        let mut shell_output = ShellOutput::default();
        shell_output.set_stdout(String::from("testing"));
        assert_eq!(shell_output.stdout, Some(String::from("testing")))
    }
}
