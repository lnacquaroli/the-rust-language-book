// To panic! or not to panic!

// Depends on the situation and the decision you make in the end.
// When the code panics, there's no way to recover.
// If you choose to return a Result value, you give the calling code some options.
// It can either search to recover depending on the situation and could also Err in the end if all the options are exhausted, using panic!.

// Final code
// Seems that returning Result is a good default choice to give more options before panic!.

// Prototyping and test
// However, in prototype code and tests seems panic to be more appropriate though.
// In a test you want the whole test to fail if one piece fails.
// panic! is how a test is marked as a failure.
// unwrap and except are also handy methods when prototyping.

// Usually you want to panic! when some assumption, guaranteem contract, or invariant has been broken.
// Invalid values, contraditory values, missing values.
// Calling a function with the wrong input for instance, can cause vulnerabilities in your system if you try to parse it after handling them.
// Static typing is one way to ensure error handling by the compiler and reduces verbosity.
// Having a function that accepts a type rather than an Option ensures you cannot skip the input, since you are expecting something, an not Some or None.

use std::net::IpAddr;

fn main() {
    // Situations with more information than the compiler
    // If you can ensure by manually inspecting the code that you’ll never have an Err variant, it’s perfectly acceptable to call unwrap, and even better to document the reason you think you’ll never have an Err variant in the expect text. Here’s an example:
    let _home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid.");
}

// Creating custom types for validation

// Recall the guessing game in ch02: we never validated the guess was between accepted values.
// One way could be to allow i32 instead of u32 to allow potentially negative numbers.
// We can make a new type and put the validations in a function to create an instance rather than repeating the validations everywhere.

// This struct is where the value will be stored.
pub struct Guess {
    value: i32,
}

impl Guess {
    // The new associated function creates instances of Guess values.
    // It checks if the input value is valid.
    // If it doesn't pass the test, will panic.
    // If it passes the test, will return the Guess instance.
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        return Guess { value };
    }

    // The value associated method (self as input) works as a getter.
    // This method borrows self.
    // This public method is neccessary because the value field in Guess is private.
    // Is important the value field remains private so the code using Guess structs cannot set value directly, without passing the check of the associated fn new.
    pub fn value(&self) -> i32 {
        // getter method
        return self.value;
    }
}
