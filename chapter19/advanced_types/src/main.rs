/*

- Creating type synonyms with type aliases
-- Rust provides the ability to declare a type alias to give an existing type
another name. For this we use the type keyword.
-- Using this method we don't get the type checking benefits compared to the newtype
pattern.
-- The main use case for type synonyms is to reduce repetition for long definitions.
-- Type aliases are also commonly used with the Result<T, E> type for reducing
repetition.

- The never type that never returns
-- Rust has a special type named ! that’s known in type theory lingo as the empty
type because it has no values. We prefer to call it the never type because it stands
in the place of the return type when a function will never return.
-- The continue is a special never type.
-- In this code, the same thing happens as in the match in Listing 19-26: Rust sees
that val has the type T and panic! has the type !, so the result of the overall
match expression is T. This code works because panic! doesn’t produce a value; it
ends the program. In the None case, we won’t be returning a value from unwrap, so
this code is valid.
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}

- Dynamically Sized Types and the Sized Trait
-- Rust needs to know certain details about its types, such as how much space to
allocate for a value of a particular type.
-- All values of a type must use the same amount of memory.
-- This leaves one corner of its type system a little confusing at first: the
concept of dynamically sized types. Sometimes referred to as DSTs or unsized types,
these types let us write code using values whose size we can know only at runtime.
-- str (not &str) and traits are DSTs.
-- For example:
If Rust allowed us to write this code, these two str values would need to
take up the same amount of space. But they have different lengths: s1 needs
12 bytes of storage and s2 needs 15. This is why it’s not possible to create
a variable holding a dynamically sized type.

let s1: str = "Hello there!";
let s2: str = "How's it going?";

So although a &T is a single value that stores the memory address of where the T is
located, a &str is two values: the address of the str and its length. As such, we
can know the size of a &str value at compile time: it’s twice the length of a usize.
That is, we always know the size of a &str, no matter how long the string it refers
to is. In general, this is the way in which dynamically sized types are used in
Rust: they have an extra bit of metadata that stores the size of the dynamic
information.
--- The golden rule of dynamically sized types is that we must always put
values of dynamically sized types behind a pointer of some kind.
-- Rust provides the Sized trait to determine whether or not a type’s size is known
at compile time. This trait is automatically implemented for everything whose size
is known at compile time.

*/

use std::{fmt, io::Error};

// Kilometers is an alias of the i32 type
type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;

fn main() {
    // Using aliases
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    let _f: Thunk = Box::new(|| println!("hi"));

    // Never types
    // Inside a for loop, within a match statement (only single type return allowed)
    // let guess: u32 = match guess.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => continue, // the never type continue can be combined here
    // };

    // Dynamically sized types (str): the following is not allowed
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";
}

// Consider this trait:
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}

// Now with type alias to remove Result<..., Error> repetitions:
type Result2<T> = std::result::Result<T, std::io::Error>;
pub trait Write2 {
    fn write(&mut self, buf: &[u8]) -> Result2<usize>;
    fn flush(&mut self) -> Result2<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result2<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result2<()>;
}
