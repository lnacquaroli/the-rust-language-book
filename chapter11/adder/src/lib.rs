/* Organisation of the tests

The Rust community thinks in terms of two main categories: unit and integration tests.

Unit tests are small and more focused, testing one module in isolation at a time, and can
test private interfaces.

Integration tests are entirely external to your library and use your code in the same way
any other external code would, using only the public interface and potentially exercising
multiple modules per test.

Unit tests:
In the src directory in each file with the code that they’re testing.
The convention is to create a module named tests in each file to contain the test functions
and to annotate the module with cfg(test).

Integration tests:
To create integration tests, you first need a tests directory.
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs

Each file in the tests directory is a separate crate.
To run all the tests in a particular integration test file, use the --test argument of
cargo test followed by the name of the file.

To create multiple files (crates) in the integration tests that share helper functions
we would create a folder inside tests, named common:
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs

Integration Tests for Binary Crates
If our project is a binary crate that only contains a src/main.rs file and doesn’t have
a src/lib.rs file, we can’t create integration tests in the tests directory and bring
functions defined in the src/main.rs file into scope with a use statement.
Only library crates expose functions that other crates can use; binary crates are meant to
be run on their own.

This is one of the reasons Rust projects that provide a binary have a straightforward
src/main.rs file that calls logic that lives in the src/lib.rs file.
Using that structure, integration tests can test the library crate with use to make the
important functionality available.
If the important functionality works, the small amount of code in the src/main.rs file
will work as well, and that small amount of code doesn’t need to be tested.
*/

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
