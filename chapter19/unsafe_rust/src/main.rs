/*

Unsafe Rust

- Exists because, by nature, static analysis is conservative.
- The other reason is because the underlying computer hardware is unsafe.
- Rust allows to do low-level systems programming, such as interacting with the OS.
- You do it at your own risk.
- It does not turn off all Rust safe checks.

- Unsafe superpowers (cannot do in safe Rust)
-- Dereference a raw pointer
-- Call an unsafe function or method
-- Access or modify a mutable static variable
-- Implement an unsafe trait
-- Access fields of unions

- Raw pointers
-- Unsafe Rust has two new types called raw pointers that are similar to references.
-- Can be immutable or mutable and are written as *const T and *mut T, respectively.
(The asterisk isn’t the dereference operator; it’s part of the type name.)
-- In the context of raw pointers, immutable means that the pointer can’t be directly
assigned to after being dereferenced.
-- Are allowed to ignore the borrowing rules by having both immutable and mutable
pointers or multiple mutable pointers to the same location
-- Aren’t guaranteed to point to valid memory
-- Are allowed to be null
-- Don’t implement any automatic cleanup

- Global variables are called static in Rust
-- They can be problematic with ownership rules
-- Are written in SCREAMING_SNEAK_CASE format by convention
-- Can only store references with the 'static lifetime

- Global vs Constant
-- Values in a static variable have a fixed address in memory. Using the value will
always access the same data.
-- Constants, on the other hand, are allowed to duplicate their data whenever
they’re used.
-- Another difference is that static variables can be mutable. Accessing (reading)
and modifying (writing) mutable static variables is unsafe.


- Unsafe traits
-- A trait is unsafe when at least one of its methods has some invariant that the
compiler can’t verify.
-- We must uphold the invariants the compiler cannot verify.

- Unions
-- A union is similar to a struct, but only one declared field is used in a
particular instance at one time. Unions are primarily used to interface with unions
in C code.
-- Accessing union fields is unsafe because Rust can’t guarantee the type of the
data currently being stored in the union instance. You can learn more about unions
in the Rust Reference.

*/

use core::slice;

// Global variables are called static in Rust
static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn main() {
    // Dereferencing

    // You can create immutable raw pointer without the unsafe keyword
    // With raw pointers, we can create a mutable pointer and an immutable pointer
    // to the same location and change data through the mutable pointer, potentially
    // creating a data race. Be careful!
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // Creating a raw pointer to an arbitrary memory address (don't)
    let address = 0x012345usize;
    let _r = address as *const i32;

    // Dereference only inside unsafe block
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // Unsafe function
    unsafe {
        dangerous();
    }

    // Safe abstraction over unsafe code
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (c, d) = split_at_mut(r, 3);
    assert_eq!(a, c);
    assert_eq!(b, d);

    // Using extern function (to call other languages functions into Rust)
    // This allows also to call Rust functions in other languages
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // Accessing or modifying mutable static variables
    println!("name is: {}", HELLO_WORLD);

    // Accessing and modifying mutable static variables is unsafe.
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// Unsafe function or method
unsafe fn dangerous() {}

// We cannot implement the split_at_mut directly
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    // Slices are pointers to some data and the length of the slice
    let len = values.len(); // access to the length of the slice
    let ptr = values.as_mut_ptr(); // unsafe mutable pointer to the data

    assert!(mid <= len);

    // Rust’s borrow checker can’t understand that we’re borrowing different parts
    // of the slice; it only knows that we’re borrowing from the same slice twice.
    //(&mut values[..mid], &mut values[mid..])

    // Unsafe abstraction inside the function
    // Unsafe raw pointer function: slice::from_raw_parts_mut
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// Unsafe traits
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
