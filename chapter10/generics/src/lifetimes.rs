/*
Lifetimes - basics

It's a concept that is not found in other common languages.

Lifetimes are another kind of generic.
Rather than ensuring that a type has the behavior we want, lifetimes ensure that
references are valid as long as we need them to be.
Every reference in Rust has a lifetime.
Rust requires us to annotate the relationships using generic lifetime parameters to
ensure the actual references used at runtime will definitely be valid.

Preventing dangling references with lifetimes
This is the main aim of lifetimes
It cause a program to reference data other than data it's intended to reference.
This won't work
let r; // lifetime of r ('a) begins
{
    // Inner scope
    let x = 5; // lifetime of x ('b) begins
    r = &x; // Referencing x in the inner scope
} // x is out of scope now, // lifetime of x ('b) ends
// It cannot be used anymore
println!("r: {}", r); // lifetime of r ('a) ends

Lifetime annotation syntax
Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters
must start with an apostrophe (') and are usually all lowercase and very short, like
generic types.
We place lifetime parameter annotations after the & of a reference,
using a space to separate the annotation from the reference’s type.
Examples:
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime

One lifetime annotation by itself doesn’t have much meaning,
because the annotations are meant to tell Rust how generic
lifetime parameters of multiple references relate to each other.

Lifetimes on function or method parameters are called input lifetimes, and lifetimes on
return values are called output lifetimes.

The patterns programmed into Rust’s analysis of references are called the lifetime elision
rules. These aren’t rules for programmers to follow; they’re a set of particular cases that
the compiler will consider, and if your code fits these cases, you don’t need to write the
lifetimes explicitly.

Elision rules
The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a
reference. In other words, a function with one parameter gets one lifetime parameter: fn
foo<'a>(x: &'a i32); a function with two parameters gets two separate lifetime parameters:
fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

The second rule is that, if there is exactly one input lifetime parameter, that lifetime is
assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

The third rule is that, if there are multiple input lifetime parameters, but one of them
is &self or &mut self because this is a method, the lifetime of self is assigned to all
output lifetime parameters. This third rule makes methods much nicer to read and write
because fewer symbols are necessary.

The Static Lifetime
Denotes that the affected reference can live for the entire duration of the program. All
string literals have the 'static lifetime, which we can annotate as follows:

    let s: &'static str = "I have a static lifetime.";

The text of this string is stored directly in the program’s binary, which is always
available. Therefore, the lifetime of all string literals is 'static.

WARNING: Most of the time, an error message suggesting the 'static lifetime results from
attempting to create a dangling reference or a mismatch of the available lifetimes. In such
cases, the solution is fixing those problems, not specifying the 'static lifetime.
*/

use std::fmt::Display;

fn main() {
    // The borrow checker
    // Rust comes with a borrow checker to determine all borrows are valid.
    // The previous code fails to compile because the lifetime 'b is shorter than 'a.
    // This is called a dangling reference. To fix it:
    let x = 5;
    let r = &x;
    println!("r: {}", r);
    // Here, x has the lifetime 'b, which in this case is larger than 'a.
    // This means r can reference x because Rust knows that
    // the reference in r will always be valid while x is valid.

    // Generic lifetimes in functions
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is: {}", result);

    // This will also compile, because the shorter lifetime (string2) ends after
    // the println! macro, while the string1 is valid after the inner scope.
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is: {}", result);
    }

    /*
    The following will not compile: result is used after the shorter lifetime ends.
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
    */

    // Struct: lifetimes annotation
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // Generic Type Parameters, Trait Bounds, and Lifetimes Together
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result =
        longest_with_an_announcement(string1.as_str(), string2, "Today is someone's birthday!");
    println!("The longest string is {}", result);
}

/*
We’ll write a function that returns the longer of two string slices.
We don't know the lifetimes of the references that are passed in.
We don't know whether the if or else will be executed.
We can't look at the scopes as above.
The borrow checker cannot determine this either.
Lifetime annotation syntax: they don't modify the lifetimes, just describe the
relationships of the lifetimes of multiple references.
We want the signature to express the following constraint: the returned reference will
be valid as long as both the parameters are valid. This is the relationship between
lifetimes of the parameters and the return value.
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}

/*
Thinking in terms of Lifetimes
Returning always the first parameter in the longest function, we can just specify the
lifetime of the first parameter, as the second parameter lifetime does not relate to the
first's one.
*/
fn longest_first<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

/*
When returning a reference from a function, the lifetime parameter for the return type
needs to match the lifetime parameter for one of the parameters. If the reference returned
does not refer to one of the parameters, it must refer to a value created within this
function. However, this would be a dangling reference because the value will go out of
scope at the end of the function. Consider this attempted implementation of the longest
function that won’t compile:
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
*/

/*
Lifetimes annotations in struct definitions
This annotation means an instance of ImportantExcerpt can’t outlive the reference it
holds in its part field.
*/
struct ImportantExcerpt<'a> {
    part: &'a str,
}

/*
Lifetime annotations in method definitions

*/
impl<'a> ImportantExcerpt<'a> {
    // First elision rule, no lifetime annotation for self.
    fn level(&self) -> i32 {
        return 3;
    }

    /*
    Thrid elision rule:
    There are two input lifetimes, so Rust applies the first lifetime elision rule and
    gives both &self and announcement their own lifetimes. Then, because one of the
    parameters is &self, the return type gets the lifetime of &self.
    */
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// Generic Type Parameters, Trait Bounds, and Lifetimes Together
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
