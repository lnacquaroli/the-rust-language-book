// Unrecoverable Errors with panic!

// By default, when a panic occurs, the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters.
// You can opt not to clean up the variables in memory by adding a section to your Cargo.toml.
// [profile.release]
// panic = 'abort'
// Memory that the program was using will then need to be cleaned up by the operating system.

// Unlike in other languages (C), to protect your program from some sort of vulnerability, if you try to read an element at an index that doesn’t exist, Rust will stop execution and refuse to continue.

fn main() {
    // Let's call panic!
    //panic!("Crash and burn");

    // Using a panic! backtrace.
    // To get the backtrace in the terminal: > RUST_BACKTRACE=1 cargo run.
    // The output shows the backtrace with the --debug flag as default enabled.
    // To disable it you need to pass the cargo run/build --release flag.
    let v = vec![1, 2, 3];
    v[99]; // accessing an out of bound element (100th)
}
