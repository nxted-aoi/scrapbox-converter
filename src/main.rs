pub use ast::*;
use parser::page;
use visitor::{
    markdown::{MarkdownGen, MarkdownPass},
    Visitor,
};

mod ast;
mod parser;
mod visitor;

fn main() {
    let input = "[Hello World]";

    let (_, mut p) = page(input).unwrap();
    let mut pass = MarkdownPass {
        h1_level: 3,
        bold_to_h: true,
    };
    pass.visit(&mut p);
    let mut visitor = MarkdownGen::new();

    let markdown = visitor.generate(&mut p);
    println!("{markdown}");
}
