/*

Box smart pointer

- When to use them:
-- When you have a type whose size can’t be known at compile time and you want to use a
value of that type in a context that requires an exact size.
-- When you have a large amount of data and you want to transfer ownership but ensure the
data won’t be copied when you do so.
-- When you want to own a value and you care only that it’s a type that implements a
particular trait rather than being of a specific type.

- Allow you to store data on the heap rather than the stack.
- What remains on the stack is the pointer to the heap data.
- Allow accessing the value of the variable as usual (stack), while on the heap.
- It's dropped (deallocated) in stack and heap when goes out of scope, like a variable.

The Box<T> type is a smart pointer because it implements
- the Deref trait, which allows Box<T> values to be treated like references.
- the Drop trait, when a Box<T> value goes out of scope, the heap data that the box is
pointing to is cleaned up.

- Deref coercion converts a reference to a type that implements the Deref trait into a
reference to another type.
- For example, deref coercion can convert &String to &str because String implements the
Deref trait such that it returns &str.
- Works only on types that implement the Deref trait.
- The deref coercion feature also lets us write more code that can work for either
references or smart pointers.
- The number of times that Deref::deref needs to be inserted is resolved at compile time,
so there is no runtime penalty for taking advantage of deref coercion.

Mutability

- Rust does deref coercion when it finds types and trait implementations in three cases:
-- From &T to &U when T: Deref<Target=U>
-- From &mut T to &mut U when T: DerefMut<Target=U>
-- From &mut T to &U when T: Deref<Target=U>

- The first two cases are the same as each other except that the second implements mutability.
- The first case states that if you have a &T, and T implements Deref to some type U, you
can get a &U transparently.
- The second case states that the same deref coercion happens for
mutable references.
- The third case is trickier:
-- Rust will also coerce a mutable reference to an immutable one.
-- immutable references will never coerce to mutable references.
- Because of the borrowing rules, if you have a mutable reference, that mutable reference
must be the only reference to that data (otherwise, the program wouldn’t compile).

*/

use std::ops::Deref;

use crate::List::{Cons, Nil};

fn main() {
    // Store a Box into a variable. The value 5 is stored in the heap.
    let b = Box::new(5);
    println!("Box b = {}", b);

    // Recursive List with Box
    // List value will take up the size of an i32 plus the size of a box’s pointer data.
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("List: {:#?}", list);

    // Dereference a variable to assert
    let x = 5;
    let y = &x; // reference
    assert_eq!(5, x);
    assert_eq!(5, *y); // assert_eq!(5, y) errors because integer != &integer

    // Using Box like a reference
    let x = 5;
    // Instance of a copied value of x, not a reference
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Using MyBox
    let x = 5;
    // Instance of a copied value of x, not a reference
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Deref coercion
    // Calling the hello function with &m, which is a reference to a MyBox<String> value.
    // Because we implemented the Deref trait on MyBox<T>, Rust can sturn &MyBox<String>
    // into &String by calling deref.
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // Without deref coercion, would have to pass a reference to a slice of String
    //let m = MyBox::new(String::from("Rust"));
    //hello(&(*m)[..]);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Define a custom Box type
// The MyBox type is a tuple struct with one element of type T.
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        return MyBox(x);
    }
}

// Implement the Deref trait for MyBox
impl<T> Deref for MyBox<T> {
    // The type Target = T; syntax defines an associated type for the Deref trait to use.
    // Associated types are a slightly different way of declaring a generic parameter.
    type Target = T;

    // The deref method gives the compiler the ability to take a value of any type that
    // implements Deref and call the deref method to get a & reference that it knows how to
    // dereference.
    fn deref(&self) -> &Self::Target {
        return &self.0; // returns a reference to the value
    }
}

fn hello(name: &str) {
    println!("Hello, {} !", name);
}
