mod errors;

mod lexer;
mod parser;
mod ir;
mod analysis;
mod codegen;

use std::io::Write;
use std::{io, env, fs};
use std::io::prelude::*;

fn main() {
    if let Some(_) = env::args().nth(1) {
        file();
    } else {
        panic!("Expected File")
    }
}

fn file() {
    let mut file = fs::File::open(env::args().nth(1).unwrap()).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let chars = &input.chars().collect::<Vec<_>>()[..];
    let mut lexer = lexer::Lexer::new(chars);
    let lex_results = lexer.go().unwrap();
    println!("______________________");
    println!("lexer output:");
    lex_results.iter().map(|t| println!("{}", t)).for_each(drop);

    let mut parser = parser::Parser::new(&lex_results);
    let parse_results = parser.go();
    println!("______________________");
    println!("parser output:");
    println!("{:#?}", parse_results);

    let mut unwrapped = parse_results.unwrap();
    let mut irbuilder = ir::IRBuilder::new(&unwrapped);
    let ir_results = irbuilder.go();
    println!("______________________");
    println!("IR output:");
    println!("{:#?}", ir_results);

    let mut analyzer = analysis::Analyzer::new(&mut unwrapped);
    let analysis_results = analyzer.go();
    println!("______________________");
    println!("analysis output:");
    println!("{:#?}", analysis_results);

    let mut generator = codegen::Generator::new(&unwrapped, "chi", &env::args().nth(1).unwrap());
    generator.go().expect("GENERATION ERROR");
    println!("______________________");
    println!("codegen output:");
    println!("{:#?}", generator.to_cstring());

    println!("Dumping to file...");
    let mut file_name = env::args().nth(1).unwrap();
    file_name.push_str(".ll");
    generator.dump_to_file(&file_name);
    println!("File done!");
}
