// The Slice type

// Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.

// They store a reference to the first element and a length.

// Instead if using usize types for a string, it's better to use a string slices.
// When returning a usize on its own it’s only a meaningful number in the context of the &String. Because it’s a separate value from the String, there’s no guarantee that it will still be valid in the future.
// A string slice is a reference to part of a String.
// Literals are string slices.

// Other Slices
// Slices can be used with numbers as well.
// They work the same way as string slices do, by storing a reference to the first element and a length.
// Arrays can also be sliced with the slice type.

fn main() {
    // Let's write a function that takes a string of words separated by spaces and returns the first word it finds in that string. If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.
    let _first = first_word(&String::from("hello"));

    {
        // String slices
        let s = String::from("hello world");
        let _hello = &s[0..5]; // This is a slice of s from the index 0 to 5
                               // &s[0..5] is the same as &s[..5]
        let _world = &s[6..11]; // This is a slice of s from the index 6 to 11
                                // let len = s.len()
                                // &s[3..len] is the same as &s[3..]

        // You can also take a slice of the entrire string with any of the following:
        // let slice = &s[0..len]
        // let sice = &s[..]
    }

    // With a string slice
    let _first_2 = first_word_2(&String::from("hello"));

    // String literals are slices
    // The type of s here is &str: it’s a slice pointing to that specific point of the binary. This is also why string literals are immutable; &str is an immutable reference.
    let _s = "Hello, world!";

    // String Slices as Parameters
    // A last improvement on the first_word would be to use a &str type signature in the function. This allows us to use the same function on both &String values and &str values.
    let _first_3 = first_word_3("hello"); // String literals are slices

    // Defining a function to take a string slice instead of a reference to a String makes our API more general and useful without losing any functionality:
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word_3(&my_string[0..6]);
    let word = first_word_3(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word_3(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word_3(&my_string_literal[0..6]);
    let word = first_word_3(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_3(my_string_literal);

    // Other slices
    let a = [1, 2, 3, 4, 5];
    let _slice_a = &a[1..3];
    assert_eq!(_slice_a, &[2, 3]);
}

// We now have a way to find out the index of the end of the first word in the string, but there’s a PROBLEM. We’re returning a usize on its own, but it’s only a meaningful number in the context of the &String. In other words, because it’s a separate value from the String, there’s no guarantee that it will still be valid in the future.
fn first_word(s: &String) -> usize {
    // Convert to an array of bytes
    let bytes = s.as_bytes();

    // For now, know that iter is a method that returns each element in a collection and that enumerate wraps the result of iter and returns each element as part of a tuple instead. The first element of the tuple returned from enumerate is the index, and the second element is a reference to the element. This is a bit more convenient than calculating the index ourselves.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i; // If we find the space byte, we return the position
        }
    }

    // Otherwise we return the length if the string
    return s.len();
}

// Let’s rewrite first_word to return a slice. The type that signifies “string slice” is written as &str.
// Now when we call first_word_2, we get back a single value that is tied to the underlying data. The value is made up of a reference to the starting point of the slice and the number of elements in the slice.
fn first_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // or &s[..i]
        }
    }

    return &s[..];
}

fn first_word_3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // or &s[..i]
        }
    }

    return &s[..];
}
