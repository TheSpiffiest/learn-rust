use std::env;

fn main() {
    // Nest some for loops 
    // arg-counter 3
    // would output...
    // 1 
    // 2
    // 3
    let args: Vec<String> = env::args().collect();

    // // DEBUG - What do the command line args look like?
    // println!("Command Line Args Vector is: {:?}", args);
    // println!("    {:?}", args.len());

    // Print Usage if launched from command line
    if args.len() == 1 {
        println!("Usage: arg-counter <number to count to>");
        std::process::exit(0);
    }
    let count_to_str = args[1].clone();

    // Print Usage if launched from IDE
    if args.len() == 2 && (count_to_str == "arg-counter" || count_to_str == "arg-counter.exe")  {
    
        println!("Usage: arg-counter <number to count to>");
        std::process::exit(0);
        
    }

    // Convert the string to number
    // ASSUMES parameter is a number, this can fail 
    let count_to: i32 = count_to_str.parse::<i32>().unwrap();

    // Count from 1 to count_to inclusive
    for n in 1..(count_to + 1) {
        println!("{:4}", n);
    }
}
