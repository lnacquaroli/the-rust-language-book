// Functions
// Name them using snake case style, lower cases.
// Multiple functions can be defined anywahere and called by main.
// Function signature is given by the set of parameters passed to the function.
// The parameters must be set with their types (type annotation).

// Rust is an expression-based language.
// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resulting value.
// Functions definitions are also statements.
// Calling a function or a macro is an expression.
// Expressions do not include semicolon at the end. If you put one you turn it into a statemtent and it will not return a value.

fn main() {
    println!("Hello, world!");
    another_function();
    print_value_of_x(5);
    print_labeled_measurement(5, 'h');

    // Statement
    let _y = 6;
    // Not allowed in Rust, unlike Python and Julia
    //x = y = 6

    // Expressions evaluate to a value. They can be part of a statement
    let y = {
        let x = 3;
        x + 1 // Expressions do not include semicolon at the end
    }; // This block evaluates to 4 and bounds it to y
    println!("The value of y is: {y}");

    // In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function.
    let x = five();
    println!("The value of x is: {x}");

    let z = plus_one(5);
    println!("The value of z is: {z}");
}

fn another_function() {
    println!("Another function.");
}

fn print_value_of_x(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value} {unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1 // If you put a semicolon you'll get a error, unless you use return
}
