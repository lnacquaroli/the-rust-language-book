/*

Reference cycles

- We can see that Rust allows memory leaks by using Rc<T> and RefCell<T>: it’s possible to
create references where items refer to each other in a cycle. This creates memory leaks
because the reference count of each item in the cycle will never reach 0, and the values
will never be dropped.

Creating reference cycles is not easily done, but it’s not impossible either. If you have
RefCell<T> values that contain Rc<T> values or similar nested combinations of types with
interior mutability and reference counting, you must ensure that you don’t create cycles;
you can’t rely on Rust to catch them.

So far, we’ve demonstrated that calling Rc::clone increases the strong_count of an Rc<T>
instance, and an Rc<T> instance is only cleaned up if its strong_count is 0. You can also
create a weak reference to the value within an Rc<T> instance by calling Rc::downgrade and
passing a reference to the Rc<T>. Strong references are how you can share ownership of an
Rc<T> instance. Weak references don’t express an ownership relationship, and their count
doesn’t affect when an Rc<T> instance is cleaned up. They won’t cause a reference cycle
because any cycle involving some weak references will be broken once the strong reference
count of values involved is 0.

When you call Rc::downgrade, you get a smart pointer of type Weak<T>. Instead of increasing
the strong_count in the Rc<T> instance by 1, calling Rc::downgrade increases the weak_count
by 1. The Rc<T> type uses weak_count to keep track of how many Weak<T> references exist,
similar to strong_count. The difference is the weak_count doesn’t need to be 0 for the
Rc<T> instance to be cleaned up.

Because the value that Weak<T> references might have been dropped, to do anything with the
value that a Weak<T> is pointing to, you must make sure the value still exists. Do this by
calling the upgrade method on a Weak<T> instance, which will return an Option<Rc<T>>.
You’ll get a result of Some if the Rc<T> value has not been dropped yet and a result of
None if the Rc<T> value has been dropped. Because upgrade returns an Option<Rc<T>>, Rust
will ensure that the Some case and the None case are handled, and there won’t be an invalid
pointer.

Example: Tree data structure.

All of the logic that manages the counts and value dropping is built into Rc<T> and Weak<T>
and their implementations of the Drop trait. By specifying that the relationship from a
child to its parent should be a Weak<T> reference in the definition of Node, you’re able to
have parent nodes point to child nodes and vice versa without creating a reference cycle
and memory leaks.

*/

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());

    // Example
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // Changes to strong_count to weak_count
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

#[derive(Debug)]
enum List {
    // modify the List value a Cons variant is pointing to
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

// We want a Node to own its children, and we want to share that ownership with variables so
// we can access each Node in the tree directly. To do this, we define the Vec<T> items to
// be values of type Rc<Node>. We also want to modify which nodes are children of another
// node, so we have a RefCell<T> in children around the Vec<Rc<Node>>.
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
