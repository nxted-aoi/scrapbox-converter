use std::{env, fs, io, path::Path};

pub use ast::*;
use parser::page;
use visitor::{
    markdown::{MarkdownGen, MarkdownGenConfig, MarkdownPass},
    Visitor,
};

mod ast;
mod parser;
mod visitor;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let file_name = &args[2];

    let contents = read_file(Path::new(&file_path), &file_name).expect("can not read file");

    // let input = "[** Hello World]";

    let (_, mut p) = page(&contents).unwrap();
    let mut pass = MarkdownPass {
        h1_level: 3,
        bold_to_h: true,
    };
    pass.visit(&mut p);
    let mut visitor = MarkdownGen::new(MarkdownGenConfig::default());

    let markdown = visitor.generate(&mut p);
    println!("{markdown}");
}

fn read_file(dir: &Path, name: &str) -> io::Result<String> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let file_name = entry.file_name();
            if file_name == name {
                let contents = fs::read_to_string(entry.path())?;
                return Ok(contents);
            }
        }
    }
    Ok("".to_string())
}
