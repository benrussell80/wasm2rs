use wasm2rs::parser::parse;
use std::env::args;
use std::fs;
use std::io::prelude::*;


fn main() {
    let inputs = args().collect::<Vec<_>>();

    if let Some(filename) = inputs.get(1) {
        if filename == "-" {
            let mut buffer = vec![];
    
            std::io::stdin().read_to_end(&mut buffer).expect("Unable to read stdin.");
    
            let context = parse(&buffer);
    
            println!("{}", context.emit_code());
        } else {
            let content = fs::read(filename).expect("Unable to read file");
    
            let context = parse(&content);
    
            println!("{}", context.emit_code());
        };
    } else {
        panic!("Missing WASM file path or \"-\" for reading from stdin.")
    }
}