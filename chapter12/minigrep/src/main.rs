/*
Developing a command line tool like grep.

The first task is to make minigrep accept its two command line arguments: the file path and
a string to search for. That is, we want to be able to run our program with cargo run, two
hyphens to indicate the following arguments are for our program rather than for cargo, a
string to search for, and a path to a file to search in, like so:
> cargo run -- searchstring example-filename.txt


*/

// To parse arguments: it returns an iterator of the arguments passed to the CLI
// For unicode support use env::args_os
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // --snip--
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing input arguments: {err}");
        process::exit(1);
    });

    //println!("Searching for {}", config.query);
    //println!("In file {}", config.file_path);

    // Handling error variant
    // The run function does not return a value to unwrap, that's why the if let
    if let Err(e) = minigrep::run(config) {
        // --snip--
        eprintln!("application error: {e}");
        process::exit(1);
    }
}
