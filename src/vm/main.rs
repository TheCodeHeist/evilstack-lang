pub mod error;
pub mod evilstack_vm;
pub mod tokenizer;

use std::env::{self, current_dir};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        writeln!(io::stderr(), "Usage: {} <filename>", args[0]).unwrap();
        std::process::exit(1);
    }

    let file_location = Path::new(&args[1]);
    let file_path = Path::new(&current_dir().unwrap()).join(file_location);

    let mut file = File::open(file_path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let mut tokenizer = tokenizer::Tokenizer::new();
    tokenizer.tokenize(&contents);

    let symbols = tokenizer.into_symbols();
    // for symbol in &symbols {
    //     println!("{:?}", symbol);
    // }

    let mut runtime = evilstack_vm::EvilStackVM::new(symbols);
    runtime.execute();
}
