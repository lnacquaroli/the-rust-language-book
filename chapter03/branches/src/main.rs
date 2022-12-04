// Control flow

// If statements
// Blocks of code in if expressions are called arms (like match)
// Conditions must be a bool
// Multiple if statements:
// Rust only executes the block for the first true condition, and once it finds one, it doesn’t even check the rest.
// Using too many if expressions leads to refactor to use match expressions.
// Values that have the potential to be results from each arm of the if must be the same type.

// loops
// There are 3 types: loop, while and for.

// loop
// The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.
// Rust also provides a way to break out of a loop using code. You can place the 'break' keyword within the loop to tell the program when to stop executing the loop.
// We also used 'continue' in the guessing game, which in a loop tells the program to skip over any remaining code in this iteration of the loop and go to the next iteration.
// loop labels to disambiguate nested loops

// while
// Usually replaces the loop, if, else, break combination in Rust.

// for loops
// Repeat sequences of code a certain amount of times
// Usually used with Range method, or the rev (reversed range)

fn main() {
    // If statements

    let number = 3;

    // Blocks of code in if expressions are called arms (like match)
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    // Conditions must be a bool
    if number != 0 {
        println!("The number was something other than zero");
    }

    // Multiple if statements
    let number = 6;

    // Rust only executes the block for the first true condition, and once it finds one, it doesn’t even check the rest.
    // Using too many if expressions leads to refactor to use match expressions.
    // Values that have the potential to be results from each arm of the if must be the same type.
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // loop
    // Returning a value after break
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // loop labels to disambiguate nested loops
    // The outer loop has the label 'counting_up, and it will count up from 0 to 2. The inner loop without a label counts down from 10 to 9. The first break that doesn’t specify a label will exit the inner loop only. The break 'counting_up; statement will exit the outer loop.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // while loop
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");

    // for loop
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // for with Range reversed
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    // Practice
    let temp_celsius = 25.0;
    let temp_fahrenheit = celsius_to_fahrenheit(temp_celsius);
    println!("{temp_celsius} C are equivalent to {temp_fahrenheit} F");

    println!("Fibonacci generator");
    println!("{}", fibonacci(1));
    println!("{}", fibonacci(3));
    println!("{}", fibonacci(5));
}

fn celsius_to_fahrenheit(temp_celsius: f64) -> f64 {
    temp_celsius * 1.8 + 32.0
}

// https://benjaminbrandt.com/fibonacci-in-rust/
fn fibonacci(x: u32) -> u32 {
    match x {
        0 => 1,
        1 => 1,
        _ => fibonacci(x - 1) + fibonacci(x - 2),
    }
}
