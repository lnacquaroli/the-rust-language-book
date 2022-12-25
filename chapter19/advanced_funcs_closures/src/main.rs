/*

- Function pointers (type fn)
-- This technique is useful when you want to pass a function you’ve already defined
rather than defining a new closure.
-- Functions coerce to the type fn (with a lowercase f), not to be confused with the
Fn closure trait.
-- The fn type is called a function pointer. Passing functions with function
pointers will allow you to use functions as arguments to other functions.
-- Function pointers implement all three of the closure traits (Fn, FnMut, and
FnOnce).
-- It’s best to write functions using a generic type and one of the closure traits
so your functions can accept either functions or closures.
-- Except when using with external languages that do not support closures (C).

- Returninig closures
-- Closures are represented by traits, so you can’t return closures directly.
-- This will not compile:
fn returns_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}
Rust doesn’t know how much space it will need to store the closure. (trait object)

*/

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
    // Closure
    let list_of_numbers = vec![1, 2, 3];
    let _list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    // Or named function with the ToString trait
    let list_of_numbers = vec![1, 2, 3];
    let _list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    // Or enums as initializer function
    let _list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

enum Status {
    Value(u32),
    Stop,
}

// Returning closures with trait objects
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
