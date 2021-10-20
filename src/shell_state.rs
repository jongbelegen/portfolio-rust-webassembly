#[derive(Default)]
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
        self.stdout = Some(stdout);
        self.stderr = None;
    }
}
