// Basics

// Every language has a way to handle the duplication of concepts.
// In Rust this is called generics.
// This allows to express the behaviour of generics or how they relate to other generics whithout knowin the actual object when compiling the code.

// Before going to generics, let's use an example on removing duplication by extracting a function.
// This technique will serve then to apply to generic function.

fn main() {
    // Find the largest number in a list: version 1.
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    // Find the largest number in two lists: version 2.
    // Twice the same block: duplication, error prone, tedious.
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    // Find the largest number in a list: version 3.
    // Use an abstraction by defining a function that operates on any list of integers.
    // Cleaner code, express the concept abstractly.
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_number(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest_number(&number_list);
    println!("The largest number is {}", result);
}

// Abstraction to find the largest number with a function.
// The largest function has a parameter called list, which represents any concrete slice of a vector of i32 values we might pass into the function.
fn largest_number(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}
