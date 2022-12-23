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

*/

use crate::List::{Cons, Nil};

fn main() {
    // Store a Box into a variable. The value 5 is stored in the heap.
    let b = Box::new(5);
    println!("Box b = {}", b);

    // Recursive List with Box
    // List value will take up the size of an i32 plus the size of a box’s pointer data.
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("List: {:#?}", list);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
