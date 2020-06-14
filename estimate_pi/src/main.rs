extern crate rand;
use rand::Rng;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
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
    let count_to: i64 = count_to_str.parse::<i64>().unwrap();

    let mut rng = rand::thread_rng();

    let mut in_count = 0;
    let mut out_count = 0;
    let mut x: f64 = rng.gen_range(-1.0,1.0);
    let mut y: f64 = rng.gen_range(-1.0,1.0);
    for _i in 1..count_to {        
        // println!("({:?},{:?})",x,y);
        if (x.powi(2) + y.powi(2)).powf(0.5) > 1.0 {
            out_count += 1;
        } else {
            in_count += 1;
        }
        x = rng.gen_range(-1.0,1.0);
        y = rng.gen_range(-1.0,1.0);
    }
    // println!("InCount {:?}, OutCount {:?}", in_count, out_count);
    println!("Pi is exactly: {:?}", f64::from(4*in_count)/f64::from(out_count+in_count));

}
