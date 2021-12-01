use wasm_bindgen::prelude::*;
use crate::shell_state::ShellState;

mod executor;
mod shell_state;
mod terminal;
mod command;
mod builtin;
mod parser;
mod utils;
mod exception;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: String);
}

#[wasm_bindgen]
pub fn run_command(name: &str) {
    let mut shell_state = ShellState::default();

    executor::run(&String::from(name), &mut shell_state);
}
