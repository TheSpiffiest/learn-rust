use std::thread;
use structopt::StructOpt;


/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
#[structopt(name = "cli-args", about = "An example of Command Line Arg usage.")]
struct CliOptions {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: bool,

    /// The pattern to look for
    hash_method: String,

    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}


use failure::ResultExt;
use exitfailure::ExitFailure;

// fn main() -> Result<(), ExitFailure> {
//     let path = "test.txt";
//     let content = std::fs::read_to_string(path)
//         .with_context(|_| format!("could not read file `{}`", path))?;
//     println!("file content: {}", content);
//     Ok(())
// }

fn do_hard_work() {
    // for i in 1..123 {
    //     let x = 1;
    // }
    thread::sleep_ms(200);

}

fn main() {
    let pb = indicatif::ProgressBar::new(100);
    let opt = CliOptions::from_args();
    println!("Debug: {:?}, Hash Method: {:?}, Path: {:?}", opt.debug, opt.hash_method, opt.path);
    let known_hash_methods = ["SHA256", "MD5", "SHA1"];
    if known_hash_methods.contains(&&opt.hash_method[..]) {
        println!("Known Hash Method: {:?}", opt.hash_method);
    } else {
        println!("Please Use a Known Hash Method: {:?}", known_hash_methods)
    }
    for i in 0..100 {
        do_hard_work();
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}