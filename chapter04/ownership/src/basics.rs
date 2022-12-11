// Ownership
// Minds the stack and the heap mems
// Many programming languages don’t require you to think about the stack and the heap very often. But in a systems programming language like Rust, whether a value is on the stack or the heap affects how the language behaves and why you have to make certain decisions.
// Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. Contemporary processors are faster if they jump around less in memory.
// When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.
// Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses.

// Ownership Rules
//  Each value in Rust has an owner.
//  There can only be one owner at a time.
//  When the owner goes out of scope, the value will be dropped.

fn main() {
    // Variable scope
    // The variable s refers to a string literal, where the value of the string is hardcoded into the text of our program. The variable is valid from the point at which it’s declared until the end of the current scope.
    let _s = "hello";

    {
        // s is not valid here, it’s not yet declared
        let _s = "hello"; // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    // The String type
    // String literals are convenient, but they aren’t suitable for every situation in which we may want to use text. One reason is that they’re immutable. Another is that not every string value can be known when we write our code. For these situations, Rust has a second string type, String. This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time. You can create a String from a string literal using the from function, like so:
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print: hello, world!

    // Why can String be mutated but literals cannot? The difference is how these two types deal with memory.
    // With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:
    //    The memory must be requested from the memory allocator at runtime.
    //    We need a way of returning this memory to the allocator when we’re done with our String.
    // Instead of a GC and handling the memory ourselves, in Rust the memory is automatically returned once the variable that owns it goes out of scope.
    // When a variable goes out of scope, Rust calls a special function for us. This function is called drop after the brackets.

    // Ways variables and data interact: Move
    // For integeres and simple values, the following do as expected: binds 5 to x and makes a copy to y
    let x = 5;
    let y = x;
    // For a String we can do the same thing, however, the result is different: in fact, the second assignment drops s1 and the value is moved to s2 now. This prevents freeing memory twice which can lead to memory corruption.
    let s1 = String::from("hello");
    let _s2 = s1;
    // println!("{}, world!", s1); // This would cause an error
    // Move can be thought as a shallow copy in other languages, copying the pointer-length-capacity, but since Rust drops the first variable, it calls it a move.
    // This way, Rust will never create a "deep" copy of the the data.

    // Ways variables and data interact: Clone
    // The clone method makes a deep copy of the data on the heap.
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // Stack-only data: Copy
    // Types such as integers that have known size at compile time are stored on the stack.
    // This means that there is no difference between a shallow and a deep copy.
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    // Types (such as integers) that implements the Copy trait do not move, they are copied and still available after assignment to another variable. This happends as long as the type does not have an implementation of the Drop trait.
    // As a rule (see docs) any simple numerical type (u32, bool, f64, etc) implements Copy. Tuples as well, as long as their elements also do.

    // Ownership and functions
    // This works the same way as the assignment of variables
    {
        let s = String::from("hello"); // s comes into scope

        takes_ownership(s); // s's value moves into the function...
                            // ... and so is no longer valid here

        // If we tried to use s after the call to takes_ownership, Rust would throw a compile-time error

        let x = 5; // x comes into scope

        makes_copy(x); // x would move into the function,
                       // but i32 is Copy, so it's okay to still
                       // use x afterward
    } // Here, x goes out of scope, then s. But because s's value was moved, nothing
      // special happens.

    // Return values and scope
    // Returning values can also transfer ownership.
    {
        let s1 = gives_ownership(); // gives_ownership moves its return
                                    // value into s1

        let s2 = String::from("hello"); // s2 comes into scope

        let s3 = takes_and_gives_back(s2); // s2 is moved into
                                           // takes_and_gives_back, which also
                                           // moves its return value into s3
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
      // happens. s1 goes out of scope and is dropped.

    // What if we want to let a function use a value but not take ownership?
    // Rust does let us return multiple values using a tuple
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    // This works perfect but it's also quite tedious. This is where references comes in.
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
