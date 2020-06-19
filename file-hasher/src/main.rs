// extern crate hex_literal;
// extern crate sha2;
extern crate ring;
extern crate data_encoding;

use std::env;
// use error_chain::error_chain;
use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read};
static PROGRAM_NAME: &'static str = "file-hasher";
    
fn is_no_parameters() -> bool {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        return true;
    }   
    let first_arg = args[1].clone();

    if args.len() == 2 && (first_arg == PROGRAM_NAME || &first_arg[0..PROGRAM_NAME.len()] == PROGRAM_NAME )  {
        return true;
    }
    return false;
}

fn print_usage() {
    println!("Usage: file-hasher <filename>");
    println!("  - prints the hash value of the contents of the given file");
}

fn get_first_parameter() -> String {
    let args: Vec<String> = env::args().collect();
    let mut result: String = String::from("");
    if args.len() == 1 {
        return result
    }
    if args.len() >= 2 {
        let first_arg = args[1].clone();
        if first_arg == PROGRAM_NAME || &first_arg[0..PROGRAM_NAME.len()] == PROGRAM_NAME  {
            return result
        } else {
            return first_arg;
        }
    } else {
        return result;
    }
}

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, &'static str> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

fn main() {
    if is_no_parameters() {
        print_usage();
        std::process::exit(0);
    }
    let file_name: String = get_first_parameter();
    if file_name > String::from(" ") {
        let input = File::open(file_name)?;
        let reader = BufReader::new(input);
        let digest = sha256_digest(reader)?;
    
        println!("SHA-256 digest is {}", HEXUPPER.encode(digest.as_ref()))
    }

}