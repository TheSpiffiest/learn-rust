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
    for i in 1..12345 {
        let x = 1;
    }
}

fn main() {
    let pb = indicatif::ProgressBar::new(100);
    let opt = CliOptions::from_args();
    println!("Debug: {:?}, Hash Method: {:?}, Path: {:?}", opt.debug, opt.hash_method, opt.path);
    for i in 0..100 {
        do_hard_work();
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}