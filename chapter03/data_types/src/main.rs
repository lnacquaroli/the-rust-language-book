// Integers
// Have types i and u, for signed and unsigned
// Can store from -(2^(n-1)) to (2^(n-1))-1 values
// n is the number of bits
// overflow handling

// Compound types
// tuples and arrays
// Tuple
// A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.
// The tuple without any values has a special name, unit. This value and its corresponding type are both written () and represent an empty value or an empty return type. Expressions implicitly return the unit value if they don’t return any other value.
// Array
// Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.
// Arrays are useful when you want your data allocated on the stack rather than the heap or when you want to ensure you always have a fixed number of elements. An array isn’t as flexible as the vector type, though. A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size. If you’re unsure whether to use an array or a vector, chances are you should use a vector.
// Arrays are more useful when you know the number of elements will not need to change.

fn main() {
    // float precision: f64 is default (roughly as fast as f32)
    let _x = 2.0;
    let _y: f32 = 3.0;

    // Operations
    // addition
    let sum = 5 + 10;
    println!("Addition {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("Difference {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("Product {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("Quotient {}", quotient);
    let floored = 2 / 3; // Results in 0
    println!("floored {}", floored);

    // remainder
    let remainder = 43 % 5;
    println!("Remainder {}", remainder);

    // Boolean
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // Character type
    let c = 'z';
    println!("c {}", c);
    let z: char = 'ℤ'; // with explicit type annotation
    println!("z {}", z);
    let heart_eyed_cat = '😻';
    print!("hear_eyed_cat {}", heart_eyed_cat);

    // Compound types

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("\ntuple {:?}", tup);

    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    // We can access a tuple element with the period notation
    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;

    // Arrays
    let a = [1, 2, 3, 4, 5];
    println!("a: {:?}", a);
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // You can specify the number of elements when defining the array
    let b: [i32; 5] = [1, 2, 3, 4, 5];

    // You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:
    let c = [3; 5];
    println!("c {:?}", c);

    // Accessing array elements
    let _first = b[0];
    let _second = b[1];

    // Indexo out of bounds
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index (can't be more than 4).");
    let mut index = String::new();
    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
