mod ast;
mod parser;
mod type_;
mod util;

use clap::Parser;
use parser::lexer::Lexer;
use std::fs::File;
use std::io::read_to_string;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Path to Amur file to compile
    #[arg(short, long)]
    file: PathBuf,
}

fn main() {
    let args = Args::parse();

    let file = match File::open(args.file) {
        Ok(f) => f,
        Err(err) => panic!("Couldn't open file {}", err),
    };

    let file_content = match read_to_string(file) {
        Ok(c) => c,
        Err(err) => panic!("Couldn't read file {}", err),
    };

    let lexer = Lexer::new(&file_content);

    for x in lexer {
        println!("{:?}", x);
    }
}
