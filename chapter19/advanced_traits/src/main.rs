/*

- Specifying Placeholder Types in Trait Definitions with Associated Types
-- Associated types connect a type placeholder with a trait such that the trait
method definitions can use these placeholder types in their signatures.
-- The implementor of a trait will specify the concrete type to be used instead of
the placeholder type for the particular implementation.
-- That way, we can define a trait that uses some types without needing to know
exactly what those types are until the trait is implemented.

- Operator overloading
-- When we use generic type parameters, we can specify a default concrete type for
the generic type.
-- This eliminates the need for implementors of the trait to specify a concrete type
if the default type works.
-- You specify a default type when declaring a generic type with the
<PlaceholderType=ConcreteType> syntax.

- Default type parameters are used mainly:
-- To extend a type without breaking existing code
-- To allow customization in specific cases most users won’t need

- Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
-- When calling methods with the same name, you’ll need to tell Rust which one you
want to use.
-- Need to call from the specific type the method to remove ambiguity.
-- Associated methods could infere the type from the self, but functions cannot.
-- Need to use syntax <Type as Trait>::function(receiver_if_method, next_arg, ...);

- Using supertraits
-- To require one trait's functionality within another trait
-- The trait your trait definition is relying on is called a supertrait of your
trait.

- Using the Newtype Pattern to Implement External Traits on External Types
-- The orphan rule that states we’re only allowed to implement a trait on a type if
either the trait or the type are local to our crate.
-- To get around this restriction using the newtype pattern, which involves creating
a new type in a tuple struct.
-- The tuple struct will have one field and be a thin wrapper around the type we
want to implement a trait for. Then the wrapper type is local to our crate, and we
can implement the trait on the wrapper.
-- Newtype is a term that originates from the Haskell programming language. There is
no runtime performance penalty for using this pattern, and the wrapper type is
elided at compile time.
-- The downside of using this technique is that Wrapper is a new type, so it doesn’t
have the methods of the value it’s holding.
-- If we wanted the new type to have every method the inner type has, implementing
the Deref trait (discussed in Chapter 15 in the “Treating Smart Pointers Like
Regular References with the Deref Trait” section) on the Wrapper to return the inner
type would be a solution.
-- If we don’t want the Wrapper type to have all the methods of the inner type—for
example, to restrict the Wrapper type’s behavior—we would have to implement just the
methods we do want manually.



*/

use std::{fmt, ops::Add};

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // Traits with methods using the same name
    // Specifying which trait’s fly method we want to call.
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    // This will print Spot, which is wrong
    println!("A baby dog is called a {}", Dog::baby_name());
    //Calling Animal::baby_name() cannot infer type because is not a method
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // Newtype pattern
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

// Specifying Placeholder Types in Trait Definitions with Associated Types
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Overloading Add to sum Point types

// Here we use the default type parameter inside the Add trait (Rhs)
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Here we define a new generic type parameter
// We want to add values in millimeters to values in meters and have the
// implementation of Add do the conversion correctly. We can implement Add for
// Millimeters with Meters as the Rhs.
struct Millimeters(u32);
struct Meters(u32);

// <Meters> is the generic type Rhs that specifies the type of the other argument
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// Supertrait
// We are specifying our trait requires the trait Display
// We can use the to_string() method this way
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// Then we implement Display on Point and satisfy the constraint that OutlinePrint
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

// Let’s say we want to implement Display on Vec<T>, which the orphan rule prevents
// us from doing directly because the Display trait and the Vec<T> type are defined
// outside our crate.
// We can make a Wrapper struct that holds an instance of Vec<T>; then we can
// implement Display on Wrapper and use the Vec<T> value
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
