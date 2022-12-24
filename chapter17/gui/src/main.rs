/*

Someone can implement a struct outside of the library to customize behaviour.

- When we wrote the library, we didn’t know that someone might add the SelectBox
type, but our Screen implementation was able to operate on the new type and draw it
because SelectBox implements the Draw trait, which means it implements the draw
method.
- This concept—of being concerned only with the messages a value responds to rather
than the value’s concrete type—is similar to the concept of duck typing in
dynamically typed languages: if it walks like a duck and quacks like a duck, then it
must be a duck!

- When we use trait objects, Rust must use dynamic dispatch.
- The compiler doesn’t know all the types that might be used with the code that’s
using trait objects, so it doesn’t know which method implemented on which type to
call.
- Instead, at runtime, Rust uses the pointers inside the trait object to know which
method to call.
- This lookup incurs a runtime cost that doesn’t occur with static dispatch. Dynamic
dispatch also prevents the compiler from choosing to inline a method’s code, which
in turn prevents some optimizations. However, we did get extra flexibility in the
code, so it’s a trade-off to consider.

Reference: flexibility vs lookup cost
- Static dispatch: when the compiler knows what method are called at compile time.
- Dynamic dispatch: when the compiler DOES NOT know what method are called at
compile time. It needs to figure them out at runtime.

*/

use gui::{Button, Draw, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    // Below will error because String doesn’t implement the Draw trait
    // let screen = Screen {
    //     components: vec![Box::new(String::from("Hi"))],
    // };
    // screen.run();
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}
