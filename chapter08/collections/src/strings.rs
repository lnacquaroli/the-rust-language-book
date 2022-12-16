// Strings

// Rust’s propensity for exposing possible errors, strings being a more complicated data structure than many programmers give them credit for, and UTF-8. These factors combine in a way that can seem difficult when you’re coming from other programming languages.

// REMEMBER THAT VALID UNICODE SCALAR VALUES MAY BE MADE UP OF MORE THAN 1 BYTE.

// Strings are implemented as a collection of bytes (characters).

// Another point about UTF-8 is that there are actually three relevant ways to look at strings from Rust’s perspective: as bytes, scalar values, and grapheme clusters (the closest thing to what we would call letters).

// Rust has only one string type in the core language, which is the string slice str that is usually seen in its borrowed form &str.

// String literals, for example, are stored in the program’s binary and are therefore string slices.

// When Rustaceans refer to “strings” in Rust, they might be referring to either the String or the string slice &str types, not just one of those types.

// String::from("Ha ha") is the same as "Ha ha".to_string

// A String can grow in size and its contents can change, just like the contents of a Vec<T>, if you push more data into it.
// You can conveniently use the + operator or the format! macro to concatenate String values.

fn main() {
    // Creating a new string

    // Empty string from the new method in String type
    let mut _s = String::new();

    // Strings literals
    let data = "initial contents"; // literals are immutable references
    let _s = data.to_string();
    let _s1 = "initial contents".to_string();

    // From the function from inside the String type
    let _s2 = String::from("initial contents");

    // Strings are UTF-8 encoded
    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");

    // Updating a String
    let mut s1 = String::from("Katrina");
    let s2 = " Crane";
    s1.push_str(s2); // push_str takes a string slice & without ownership
    println!("{}", s1);
    println!("s2 = {}", s2); // otherwise this would error

    let mut s3 = String::from("Ichabo");
    s3.push('d'); // push method takes a character and adds it to the String
    println!("{}", s3);

    // Concatenation with +
    let s1 = String::from("Hello, ");
    let s2 = String::from("Abraham!");
    // note that s1 has been moved here and can no longer be used
    // the add method + signature enforces to take a reference in the second argument
    let s3 = s1 + &s2;
    println!("{}", s3);
    // NOTE: We can only add a &str to a String
    // Above, the compiler can coerce the &String argument into a &str.
    // When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..].
    // Although let s3 = s1 + &s2; looks like it will copy both strings and create a new one, this statement actually takes ownership of s1, appends a copy of the contents of s2, and then returns ownership of the result. In other words, it looks like it’s making a lot of copies but isn’t; the implementation is more efficient than copying.

    // Multiple concatenation
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("With +: {}", s);

    // The same can be accomplished more concisely using the format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}"); // format! uses references on all parameters
    println!("With format! macro: {}", s);

    // Indexing into Strings
    // It does not work like in any other language
    // Rust strings don't support indexing: memory management, and complexity warranties
    // This below will error
    //  let s1 = String::from("hello");
    //  let h = s1[0];
    // Internal representation
    // A String is a wrapper over a Vec<u8>
    let hello = String::from("Hola");
    println!("len of 'Hola' UTF-8 is: {}", hello.len());
    let hello = String::from("Здравствуйте");
    println!("len of 'Здравствуйте' UTF-8 is: {}", hello.len()); // An index into the string’s bytes will not always correlate to a valid Unicode scalar value.

    // Slicing strings
    // Indexing into a string is often a bad idea because it’s not clear what the return type of the string-indexing operation should be: a byte value, a character, a grapheme cluster, or a string slice.
    // If you really need to you can use the [] slice.
    // Here, s will be a &str that contains the first 4 bytes of the string. Earlier, we mentioned that each of these characters was 2 bytes.
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("slice of s: {}", s);
    // let s = &hello[0..1] // This will panick, as we are slicing a char by a half

    // Iterating over strings
    // Need to be specific as to whether you want characters or bytes
    // With the chars method to split characters
    for c in "Зд".chars() {
        println!("Char: {}", c);
    }

    for b in "Зд".bytes() {
        println!("Byte: {}", b);
    }
}
