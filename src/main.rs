mod ast;

pub use ast::*;

struct Compiler {
    last_elm_end_line: u32,
    decorate: Vec<String>,
}

impl Compiler {
    fn new() -> Self {
        Self {
            last_elm_end_line: 1,
            decorate: Vec::new(),
        }
    }

    fn is_decorate_element() -> bool {
        true
    }

    fn compile() -> String {
        "".to_string()
    }
}

fn main() {
    println!("Hello, world!");
}
