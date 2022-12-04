// Variables
// Are immutable by default.
// Making variables mutables allow to change values but not type.
// Reassigning an immutable variable to another value with different type or the same type is called shadowing.

// Constants
// They are always immutable
// Cannot be part of computed values at runtime
const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;

fn main() {
    // Variables
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // Constants
    println!("Three hours in seconds is: {}", THREE_HOURS_IN_SECONDS);

    // Shadowing
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }

    println!("The value of y after the block is: {}", y);

    // Immutable variables can change type when shadowed
    let spaces = "Phoenix no Ikki";
    println!("spaces before shadowing: {}", spaces);
    let spaces = spaces.len();
    println!("spaces after shadowing: {}", spaces);

    // Not allowed:
    //let mut spaces_2 = "NA";
    //spaces_2 = spaces_2.len();
}
