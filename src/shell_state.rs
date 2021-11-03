#[derive(Default, Debug)]
pub struct ShellState {
    pub current_dir: String,
    pub output: ShellOutput,
}
#[derive(Default, Debug)]
pub struct ShellOutput {
    pub code: Option<u32>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub stdin: Option<String>,
}

impl ShellOutput {
    pub fn is_ok(&self) -> bool {
        self.code == Some(0)
    }

    pub fn set_stdout(&mut self, stdout: String) {
        self.code = Some(0);
        self.stdout = Some(stdout.clone());
        self.stderr = None;
    }

    pub fn set_stderr(&mut self, code: u32, stdout: String) {
        self.code = Some(code);
        self.stderr = Some(stdout.clone());
        self.stdout = None;
    }

    pub fn clear(&mut self) {
        self.code = None;
        self.stdout = None;
        self.stderr = None;
        self.stdin = None;
    }

    pub fn move_stdout_to_stdin(&mut self) {
        self.stdin = self.stdout.clone();
        self.stdout = None;
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
