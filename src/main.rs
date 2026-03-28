mod ast;
mod codegen;
mod lexer;
mod parser;
mod semantic;

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let input_file = &args[1];
    let output_file = if args.len() > 2 {
        args[2].clone()
    } else {
        let path = Path::new(input_file);
        let stem = path.file_stem().unwrap().to_str().unwrap();
        format!("{}.s", stem)
    };

    // Read input file
    let input = match fs::read_to_string(input_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", input_file, e);
            return;
        }
    };

    // Tokenize
    let tokens = match lexer::tokenize(&input) {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("Lexer error: {}", e);
            return;
        }
    };

    // Parse
    let program = match parser::parse(tokens) {
        Ok(program) => program,
        Err(e) => {
            eprintln!("Parser error: {}", e);
            return;
        }
    };

    // Semantic analysis
    if let Err(errors) = semantic::analyze(&program) {
        eprintln!("Semantic errors:");
        for error in errors {
            eprintln!("  {}", error);
        }
        return;
    }

    // Generate assembly
    let assembly = codegen::generate(&program);

    // Write output file
    match fs::write(&output_file, assembly) {
        Ok(_) => {
            println!("Successfully generated assembly file: {}", output_file);
            println!("To compile and link:");
            println!("  gcc -o {} {}", output_file.replace(".s", ""), output_file);
        }
        Err(e) => {
            eprintln!("Error writing file '{}': {}", output_file, e);
        }
    }
}

fn print_usage() {
    println!("C Compiler - A simple C compiler written in Rust");
    println!();
    println!("Usage:");
    println!("  ccompiler <input.c> [output.s]");
    println!();
    println!("Examples:");
    println!("  ccompiler hello.c");
    println!("  ccompiler program.c output.s");
}
