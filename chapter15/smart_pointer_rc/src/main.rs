/*

Reference counting smart pointer

- Enables a single value to have multiple ownerships.
- Keeps track of the number of references to a value to determine if is still in use.
- If there are zero references to a value, the value can be cleaned up without any
references becoming invalid.
- It allocates data on the heap to be used in different parts of the program.
- Is used when at compiled time we don't know which part will be the last one to use data.
- Is only for use in single-threaded scenarios.

- The implementation of Rc::clone doesn’t make a deep copy of all the data like most types’
implementations of clone do.
- Reference count changes depending on the scope.

- Via immutable references, Rc<T> allows you to share data between multiple parts of your
program for reading only.
-If Rc<T> allowed you to have multiple mutable references too, you might violate one of the
borrowing rules. (see RefCell)

*/

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    // We’ll create two lists that both share ownership of a third list.
    // Instead of using Box we'll use Rc.

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // Rc::clone only increments the reference count, unlike a deep copy (a.clone()).
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    // Reference count changes when c goes out of scope.
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}
