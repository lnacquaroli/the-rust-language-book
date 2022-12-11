// References and borrowing

// A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
// The referencing to a value is made by placing an & before the variable name.
// The opposite to referencing is deferencing, which is accomplished by the * symbol.
// When reference does not take ownership of a value, the value is not drop after using the reference.
// We call the action of creating a reference borrowing.
// References are immutable by default. To make them mutable we explicitly write &mut to borrow the value.
// Caveat: there can be no more than one mutable reference to a value at a time. This prevents data races, when these conditions occur:
//  Two or more pointers access the same data at the same time.
//  At least one of the pointers is being used to write to the data.
//  There’s no mechanism being used to synchronize access to the data.

// The Rules of References
//  At any given time, you can have either one mutable reference or any number of immutable references.
//  References must always be valid.

fn main() {
    // The last example we can write it this way, using the & symbol before the variable to indicate that we make a reference instead of passing the value and its ownership to the function.
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // If we try to modify somthing we borrowed, we get an error.
    // Just as variables are immutable by default, so are references.
    // We’re not allowed to modify something we have a reference to.
    // To fix this we can use a mutable reference
    let mut s = String::from("hello");
    change(&mut s);

    // Since you cannot make more than one mutable reference at a time, we can use a block to create a new scope, allowing for multiple mutable references, just not simultaneous ones:
    let mut s2 = String::from("hello");
    {
        let _r1 = &mut s2;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let _r2 = &mut s2;

    // We also cannot have a mutable reference while we have an immutable one to the same value: using &s prevents using &mut s.
    // Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used. For instance, this code will compile because the last usage of the immutable references, the println!, occurs before the mutable reference is introduced:
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem

    // let r3 = &mut s; // BIG problem before using the value of the references

    println!("r1 = {}, r2 = {}", r1, r2); // we use the values here and dropped them

    let r3 = &mut s; // no problem for the compiler using the value once the references are alreaady dropped above
    println!("r2 = {}", r3);

    // Dangling reference
    // Some languages with pointers produces dangling pointers--a pointer that references a location in memory that may have been given to someone else--by freeing some memory while preserving a pointer to that memory.
    // If you have a reference to some data, the Rust compiler will ensure that the data will not go out of scope before the reference to the data does.
    // let reference_to_nothing = dangle(); // See comments below
    // Because s is created inside dangle, when the code of dangle is finished, s will be deallocated. But we tried to return a reference to it. That means this reference would be pointing to an invalid String. That’s no good! Rust won’t let us do this.
    // The solution here is to return the String directly:
    let _reference_to_nothing = no_dangle();
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

// Adding &mut to the signature we make clear that the function will mutate the value it borrows. (Modify it)
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!

fn no_dangle() -> String {
    let s = String::from("hello");
    s // Ownership is moved out, and nothing is deallocated.
}
