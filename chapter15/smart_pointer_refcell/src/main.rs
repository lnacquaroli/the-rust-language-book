/*

RefCell type

- Interior mutability is a design pattern in Rust that allows you to mutate data even when
there are immutable references to that data; normally, this action is disallowed by the
borrowing rules. To mutate data, the pattern uses unsafe code inside a data structure to
bend Rust’s usual rules that govern mutation and borrowing. Unsafe code indicates to the
compiler that we’re checking the rules manually instead of relying on the compiler to check
them for us.

- Represents single ownership over the data it holds, unlike Rc.
- Unlike Box, with RefCell:
- At any given time, you can have either (but not both):
-- one mutable reference OR
-- any number of immutable references.
- References must always be valid.
- With RefCell type the borrowing rules are enforce at runtime rather than at compile time.
- With references, if you break these rules, you’ll get a compiler error.
- With RefCell<T>, if you break these rules, your program will panic and exit.

- The RefCell<T> type is useful when you’re sure your code follows the borrowing rules but
the compiler is unable to understand and guarantee that.
- Is only for use in single-threaded scenarios like Rc.


- Recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:

-- Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
-- Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only
immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows
checked at runtime.
-- Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value
inside the RefCell<T> even when the RefCell<T> is immutable.

- A common way to use RefCell<T> is in combination with Rc<T>.
- Recall that Rc<T> lets you have multiple owners of some data, but it only gives immutable
access to that data.
- If you have an Rc<T> that holds a RefCell<T>, you can get a value that can have multiple
owners and that you can mutate!

*/

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // Interior mutability
    let x = 5;
    // let y = &mut x; // not allowed with immutable values.

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
