use std::env;

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

fn main() {
    if is_no_parameters() {
        print_usage();
    }

}