use std::fs;
use std::io;
use std::io::Read;

extern crate clap;
use clap::Parser;

/// Argument input for Advent of Code
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Read input from command line
    #[arg(short, long)]
    interactive: bool,

    /// Name on filename to input from
    #[arg(short, long)]
    filename: String,
}

fn prompt_input(prompt: String) -> Result<String, String> {
    println!("{prompt}: (^D (^Z on Windows) to end)");
    let mut buf : String = String::new();
    match io::stdin().lock().read_to_string(&mut buf) {
        Ok(_) => Ok(buf),
        Err(e) => Err(e.to_string())
    }
}

fn read_input_file(filepath : String) -> Result<String, String> {
    match fs::read_to_string(filepath) {
        Ok(s) => Ok(s),
        Err(e) => Err(e.to_string())
    }
}

pub fn get_input(prompt: String) -> Result<String, String> {
    let args = Args::parse();
    if args.interactive {
        prompt_input(prompt)
    } else if args.filename != "" {
        Err("Must specify --filename if not --interactive".to_string())
    } else {
        read_input_file(args.filename)
    }
}

pub fn print_hello() {
    println!("Hello, Library!");
}