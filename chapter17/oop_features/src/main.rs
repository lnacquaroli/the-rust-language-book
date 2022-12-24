/*

OOP features in Rust

- Rust is not OOP oriented per se.
- As with functional programming, is the other paradigm for pattern deisgn in Rust.
- Enums and structs with their methods can be thought as objects but they are not.

- Encapsulation (implementation details are not accessible to code) can be control
with the pub keyword for instance, on modules, functions and methods. (lib.rs)

- Inheritance is a mechanism whereby an object can inherit elements from another
object’s definition, thus gaining the parent object’s data and behavior without you
having to define them again. Rust does not implement inheritance over its structs,
except using a macro. You can do this in a limited way in Rust using the default
trait method implementations.

- Rust instead uses generics to abstract over different possible types and trait
bounds to impose constraints on what those types must provide. This is sometimes
called bounded parametric polymorphism.


*/

fn main() {
    println!("Hello, world!");
}
