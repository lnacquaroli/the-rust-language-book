// Traits

// A trait defines functionality a particular type has and can share with other types.
// We can use traits to define shared behavior in an abstract way.
// We can use trait bounds to specify that a generic type can be any type that has certain behavior.

// Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

// We want to make a media aggregator library crate named aggregator that can display summaries of data that might be stored in a NewsArticle or Tweet instance. To do this, we need a summary from each type, and we’ll request that summary by calling a summarize method on an instance.

// Trait Summary
// Each type implementing this trait must provide its own custom behavior for the body of the method. The compiler will enforce that any type that has the Summary trait will have the method summarize defined with this signature exactly.
// A trait can have multiple methods in its body: the method signatures are listed one per line and each line ends in a semicolon.

// One restriction to note is that we can implement a trait on a type only if at least one of the trait or the type is local to our crate.
// This restriction is part of a property called coherence, and more specifically the orphan rule, so named because the parent type is not present. This rule ensures that other people’s code can’t break your code and vice versa. Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which implementation to use.

// Note that it isn’t possible to call the default implementation from an overriding implementation of that same method.

use std::fmt::{Debug, Display};

pub trait Summary {
    // method summarize
    fn summarize(&self) -> String;
}

// Default implementation
// Sometimes it’s useful to have default behavior for some or all of the methods in a trait instead of requiring implementations for all methods on every type.
pub trait SummaryDefault {
    // method summarize
    fn summarize_default(&self) -> String {
        return String::from("(Read more...)");
    }
}

// Implementing a trait on a type

// Implementation of the Summary trait on the NewsArticle struct that uses the headline, the author, and the location to create the return value of summarize.
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        return format!("{}, by {} ({})", self.headline, self.author, self.location);
    }
}

// Default implementation
impl SummaryDefault for NewsArticle {}

// For the Tweet struct, we define summarize as the username followed by the entire text of the tweet, assuming that tweet content is already limited to 280 characters.
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        return format!("{}: {}", self.username, self.content);
    }
}

// Default implementations can call other methods in the same trait
pub trait SummaryMethods {
    fn summarize_author(&self) -> String;

    fn summarize_call_method(&self) -> String {
        return format!("(Read more from {}...)", self.summarize_author());
    }
}

impl SummaryMethods for Tweet {
    fn summarize_author(&self) -> String {
        return format!("@{}", self.username);
    }
}

// Traits as parameters
// The input parameter accepts any type that implements the specified trait.
// We can call notify and pass in any instance of NewsArticle or Tweet. Code that calls the function with any other type, such as a String or an i32, won’t compile because those types don’t implement Summary.
pub fn notify(item: &impl Summary) {
    return println!("Breaking news! {}", item.summarize());
}

// Trait bounds
// To enforce both items have the samve type
pub fn notify_two<T: Summary>(item1: &T, item2: &T) {}

// Specifying multiple trait bounds with +
pub fn notify_multiple(item: &(impl Summary + Display)) {}
pub fn notify_generic<T: Summary + Display>(item: &T) {}
// With the two trait bounds specified, the body of notify can call summarize and use {} to format item.

// Clearer Trait Bounds with where Clauses
// Julia comes again...
fn some_function_1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    unimplemented!()
}
fn some_function_2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    unimplemented!()
}

// Returning types that implement traits
// The ability to specify a return type only by the trait it implements is especially useful in the context of closures and iterators.
// Closures and iterators create types that only the compiler knows or types that are very long to specify.
// The impl Trait syntax lets you concisely specify that a function returns some type that implements the Iterator trait without needing to write out a very long type.
// However, you can only use impl Trait if you’re returning a single type.
fn returns_summarizable() -> impl Summary {
    return Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
}

// Using Trait Bounds to Conditionally Implement Methods
// By using a trait bound with an impl block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits.
// For example, the type Pair<T> always implements the new function to return a new instance of Pair<T> (recall from the “Defining Methods” section of Chapter 5 that Self is a type alias for the type of the impl block, which in this case is Pair<T>).
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// But in the next impl block, Pair<T> only implements the cmp_display method if its inner type T implements the PartialOrd trait that enables comparison and the Display trait that enables printing.
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
