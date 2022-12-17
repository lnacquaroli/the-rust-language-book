// Recoverable Errors with Result

// Sometimes, when a function fails, it’s for a reason that you can easily interpret and respond to. For example, if you try to open a file and that operation fails because the file doesn’t exist, you might want to create the file instead of terminating the process.

// The Result<T, E> enum has two variants to handle this: Ok(T) and Err(E).
// Both types T and E are generic, allowing flexible handling of errors.

// The Result enum (like the Option enum) is brought into scope by the prelude.

// We use expect in the same way as unwrap: to return the file handle or call the panic! macro (with the message insinde the expect method).
// expect is a preferred method over unwrap in production-quality code.

// Where The ? Operator Can Be Used
// The ? operator can only be used in functions whose return type is compatible with the value the ? is used on.
// It performs an early return of a value our of the function just like match.
// The return type of the function has to be a Result, compatible with this return.

// The ? operator won’t automatically convert a Result to an Option or vice versa; in those cases, you can use methods like the ok method on Result or the ok_or method on Option to do the conversion explicitly.

// So far, all the main functions we’ve used return (). The main function is special because it’s the entry and exit point of executable programs, and there are restrictions on what its return type can be for the programs to behave as expected.
// Luckily, main can also return a Result<(), E>. Listing 9-12 has the code from Listing 9-10 but we’ve changed the return type of main to be Result<(), Box<dyn Error>> and added a return value Ok(()) to the end. This code will now compile:
// use std::error::Error;
// use std::fs::File;
// fn main() -> Result<(), Box<dyn Error>> {
//     let greeting_file = File::open("hello.txt")?;
//     Ok(())
// }
// When a main function returns a Result<(), E>, the executable will exit with a value of 0 if main returns Ok(()) and will exit with a nonzero value if main returns an Err value. Executables written in C return integers when they exit: programs that exit successfully return the integer 0, and programs that error return some integer other than 0. Rust also returns integers from executables to be compatible with this convention.
// The main function may return any types that implement the std::process::Termination trait, which contains a function report that returns an ExitCode. Consult the standard library documentation for more information on implementing the Termination trait for your own types.

use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

fn main() {
    // Let’s call a function that returns a Result value because the function could fail.
    let greeting_file_result = File::open("hello.txt");
    // Since the returned type is a Result<T, E> the compiler won't complain.
    // The implementation of File::open with the type of the success value, std::fs::File, is a file handle.
    // The type of E used in the error value is std::io::Error.
    // Both options go into the Result enum to handle properly.
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    // Matching on different errors
    // If we want to take different actions for different failure reasons, we can do:
    let greeting_file_result = File::open("hello.txt");
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // The enum io::ErrorKind is provided by the standard library and has variants representing the different kinds of errors that might result from an io operation. The variant we want to use is ErrorKind::NotFound, which indicates the file we’re trying to open doesn’t exist yet. So we match on greeting_file_result, but we also have an inner match on error.kind().
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // The same result as above with the match expressions can be achieved using closures and the unwrap_or_else method.
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Shortcuts for panic and error: unwrap and expect
    // Using match works well enough, but it can be a bit verbose and doesn’t always communicate intent well. The Result<T, E> type has many helper methods defined on it to do various, more specific tasks. The unwrap method is a shortcut method implemented just like the match expression. If the Result value is the Ok variant, unwrap will return the value inside the Ok. If the Result is the Err variant, unwrap will call the panic! macro for us. Here is an example of unwrap in action:
    let _greeting_file = File::open("hello.txt").unwrap();

    // Similarly, the expect method lets us also choose the panic! error message. Using expect instead of unwrap and providing good error messages can convey your intent and make tracking down the source of a panic easier. The syntax of expect looks like this:
    let _greeting_file =
        File::open("hello.txt").expect("hello.txt should be includede in this project.");
}

// When a function’s implementation calls something that might fail, instead of handling the error within the function itself, you can return the error to the calling code so that it can decide what to do. This is known as propagating the error and gives more control to the calling code, where there might be more information or logic that dictates how the error should be handled than what you have available in the context of your code.
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    return match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    };
}

// This pattern of propagating errors is so common in Rust that Rust provides the question mark operator ? to make this easier.
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    // If the value is an Err this will be returned, otherwise the Ok below it
    username_file.read_to_string(&mut username)?;
    return Ok(username);
}

// We can make even shorter the function above by using the ? operator twice
fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut username = String::new();
    // If the value is an Err this will be returned, otherwise the Ok below it
    File::open("hello.txt")?.read_to_string(&mut username)?;
    return Ok(username);
}

// Finalsly we an make a minimalistic function with the same functionality
// Reading a file into a string is a fairly common operation, so the standard library provides the convenient fs::read_to_string function that opens the file, creates a new String, reads the contents of the file, puts the contents into that String, and returns it. Of course, using fs::read_to_string doesn’t give us the opportunity to explain all the error handling, so we did it the longer way first.
fn read_username_from_file_4() -> Result<String, io::Error> {
    return fs::read_to_string("hello.txt");
}
