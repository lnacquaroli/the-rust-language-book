// Generic data types

// We use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types.

// The good news is that using generic types won't make your program run any slower than it would with concrete types.

// Rust accomplishes this by performing monomorphization of the code using generics at compile time. Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

fn main() {
    // Generic types in functions

    // Calling many functions for each type:
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(*result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(*result, 'y');

    // Calling one function with a generic type
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // Generic types in struct definitions
    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };
    // Cannot mix types for the Point define below
    //let wont_work = Point { x: 5, y: 1.5 };
    // With a new PointAlt will
    let _both_integer = PointAlt { x: 5, y: 10 };
    let _both_float = PointAlt { x: 1.0, y: 4.0 };
    let _integer_and_float = PointAlt { x: 5, y: 4.0 };

    // Generic in methods definitions
    let p = Point2 { x: 5, y: 10 };
    println!("p.x = {}, p.y = {}", p.x(), p.y());

    // Mix types in structs and methods
    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// Multiple fn for different types
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

// One function with generic type
// We read this definition as: the function largest is generic over some type T. This function has one parameter named list, which is a slice of values of type T. The largest function will return a reference to a value of the same type T.
// We need to restrict the generic type T to thosw which can be ordered.
// We use the PartialOrd trait from the std library for this.
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

// Struct with one generic type
struct Point<T> {
    x: T,
    y: T,
}

// Struct with two generic types
struct PointAlt<T, U> {
    x: T,
    y: U,
}

// Generic in Enum definitions
enum Option<T> {
    Some(T), // only variant Some has the type
    None,
}

// Enum with multiple generic types
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Generic in methods definitions
struct Point2<T> {
    x: T,
    y: T,
}

impl<T> Point2<T> {
    // Getter method for x
    fn x(&self) -> &T {
        return &self.x;
    }

    // Getter method for x
    fn y(&self) -> &T {
        return &self.y;
    }
}

// We can also specify constraints on generic types when defining methods on the type. We could, for example, implement methods only on Point<f32> instances rather than on Point<T> instances with any generic type.
// This implements a specific additional method for the f32 type, along with the x and y.
impl Point2<f32> {
    fn distance_from_origin(&self) -> f32 {
        // Distance from 0.0, 0.0
        return (self.x.powi(2) + self.y.powi(2)).sqrt();
    }
}

// Generic type parameters in a struct definition aren’t always the same as those you use in that same struct’s method signatures. The code below uses the generic types X1 and Y1 for the Point struct and X2 Y2 for the mixup method signature to make the example clearer. The method creates a new Point instance with the x value from the self Point (of type X1) and the y value from the passed-in Point (of type Y2).
struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

// The purpose of this example is to demonstrate a situation in which some generic parameters are declared with impl and some are declared with the method definition. Here, the generic parameters X1 and Y1 are declared after impl because they go with the struct definition. The generic parameters X2 and Y2 are declared after fn mixup, because they’re only relevant to the method.
impl<X1, Y1> Point3<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        return Point3 {
            x: self.x,
            y: other.y,
        };
    }
}
