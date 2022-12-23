/*

Drop trait

- Lets you customize what happens when a value is about to go out of scope.
- The Drop trait is almost always used when implementing a smart pointer (Box<T>).
- You can specify that a particular bit of code be run whenever a value goes out of scope,
and the compiler will insert this code automatically.
- Drop implementation takes a mutable reference to self.

- The implemented drop method is a destructor but is difficult to call it manually.
- For an early cleanup, you can call the std::mem:drop method on a certain smart pointer.

*/

use std::mem::drop;

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");
    // After the println statement, the drop method is called to clean up the used code.
    // It does it here, because it's when the instances go out of scope.

    // This is not allowed
    //c.drop(); d.drop();

    // This can be done
    drop(c);
    drop(d);
    println!("Pointers cleaned up.")
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
