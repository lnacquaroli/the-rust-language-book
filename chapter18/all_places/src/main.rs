/*

- Patterns are a special syntax in Rust for matching against the structure of types,
both complex and simple. Using patterns in conjunction with match expressions and
other constructs gives you more control over a program’s control flow.

*/

fn main() {
    // Patterns appear in the arms of match expressions.
    // Match have to be exhaustive in the patterns.
    {
        let x = Some(2);
        match x {
            None => None,
            Some(i) => Some(i + 1),
        };
    }

    // if let expressions can replace match when only one option is considered.
    // They can have an else statement too.
    // Nested if let, else if, else if let are more flexible than match, since they
    // don't need the arms to be related.
    // The compiler does not check the exhaustiveness as in the match case.
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        // The shadowed age we want to compare to 30 isn’t valid until the new
        // scope starts with the curly bracket.
        if age > 30 {
            // Here age is into scope.
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // while let
    // The pop method takes the last element out of the vector and returns Some
    // (value). If the vector is empty, pop returns None. The while loop continues
    // running the code in its block as long as pop returns Some. When pop returns
    // None, the loop stops. We can use while let to pop every element off our stack.
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for loop
    // The value following the for keyword is the pattern.
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // let statements
    // Every time we use a let statement to assign a value to a variable, is a
    // pattern. It binds everything to the variable x, whatever the value is.
    let x = 5;
    println!("let x = {} is a pattern too.", x);
    // Match a tuple against a pattern.
    let (_x, _y, _z) = (1, 2, 3);
    let (_x, _, _z) = (1, 2, 3); // Ignore second value

    // Function parameters are patterns.
    print_coordinates(&(3, 5));
}

// Function parameters could also be a pattern.
fn _foo(_x: i32) {
    // Some code
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
