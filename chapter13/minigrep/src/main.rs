/*

- Improving the minigrep CLI with iterators.
- The env::args() returns an iterator already.

*/

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Passing ownership of the iterator returned from env::args to Config::build.
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // --snip--

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
