pub mod history;
mod parser;

pub fn run(line: &String) {
    history::append(line);

    for _cmd in parser::line_to_cmds(line) {

    }

}

fn execute_line() {

}
