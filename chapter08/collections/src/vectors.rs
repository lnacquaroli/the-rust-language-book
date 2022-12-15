// Collections
// Most other data types represent one specific value, but collections can contain multiple values.
// Unlike the built-in array and tuple types, the data these collections point to is stored on the HEAP, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs.
// Each kind of collection has different capabilities and costs, and choosing an appropriate one for your current situation is a skill you’ll develop over time.

// Can be vectors, strings or hashmaps (dictionaries like)

// Vectors
// Allow you to store more than one value in a single data structure.
// Can only store values of the same type.
// Put all the values next to each other in memory.
// Two ways of reading elements from a vector:
//  via indexing: first element in a collection is at index 0 in Rust.
//  using the get method: allows to handle out of bounds in a collection for instance.
//  The get returns None if this case, without panicking.
// When dropping a vector, all its elements are also dropped.

// Enums
// The variants of an enum are defined under the same enum type and can represent elements of different types.
// If the types of the variants are unknown we can use the trat object.

fn main() {
    // Vectors

    // Vec can create generic type placeholders

    // Empty vector of i32 type
    let _v: Vec<i32> = Vec::new();

    // vec! macro, with type inference
    let _v = vec![1, 2, 3];

    // Updating an empty mutable vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("{:#?}", v);

    // Reading elements of vectors
    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2]; // Reference to the third element
    println!("The third element by indexing is: {}", third);

    let third = v.get(2); // get method returns an Option<&T>
    match third {
        Some(third) => println!("The third element by using get is: {}", third),
        None => println!("There is not third element."),
    }

    // This will error
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0]; // immutable reference
    // v.push(6); // mutable reference
    // println!("The first element is: {}", first);

    // Iterate over a vector
    let v = vec![100, 32, 57];
    // Using a for loop
    for i in &v {
        println!("{}", i);
    }
    // Over mutable references to make changes to all elements
    let mut v2 = vec![100, 23, 57];
    for i in &mut v2 {
        *i += 50; // NEED TO DEREFERENCE TO CHANGE THE VALUE
        println!("{}", i);
    }
    println!("Modified vector\n {:#?}", v2);

    // Enums
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:#?}", row);

    // Dropping a vector drops its elements
    // A vector is freed when it goes out of scope.
    {
        let v = vec![1, 2, 3];
        // do some stuff
    } // v goes out of scope from here and is freed.
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
