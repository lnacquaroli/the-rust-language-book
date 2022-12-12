// Methods syntax

// Methods are similar to functions: we declare them with the fn keyword and a name, they can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else.
// Unlike functions, methods are defined within the context of a struct (or an enum or a trait object).
// Their first parameter is always self, which represents the instance of the struct the method is being called on.
// Methods can take ownership of self, borrow self immutably as we’ve done here, or borrow self mutably, just as they can any other parameter.
// Having a method that takes ownership of the instance by using just self as the first parameter is rare.
// The main reason for using methods instead of functions, in addition to providing method syntax and not having to repeat the type of self in every method’s signature, is for organization.

// Often, but not always, when we give methods with the same name as a field we want it to only return the value in the field and do nothing else. Methods like this are called getters, and Rust does not implement them automatically for struct fields as some other languages do. Getters are useful because you can make the field private but the method public and thus enable read-only access to that field as part of the type’s public API.

// Rust doesn’t have an equivalent to the -> operator as in C or C++; instead, Rust has a feature called automatic referencing and dereferencing. Calling methods is one of the few places in Rust that has this behavior.

// Methods can take multiple parameters that we add to the signature after the self parameter, and those parameters work just like parameters in functions.

// All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl. We can define associated functions that don’t have self as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with. We’ve already used one function like this: the String::from function that’s defined on the String type. (much like a staticmethod in Python)

// Each struct is allowed to have multiple impl blocks.
// This allows, for instance, further organization by topic, characteristics, etc...

// Let's modify the rectangle example using methods.

// An example program using methods

// Let's write a program that calculates the area of a rectangle.

fn main() {
    let rect_1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect_1.area()
    );

    println!("Is the width non-zero? {}", rect_1.width());

    // Methods with more parameters
    let rect_2 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect_3 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect_4 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect_2 hold rect_3? {}", rect_2.can_hold(&rect_3));
    println!("Can rect_2 hold rect_4? {}", rect_2.can_hold(&rect_4));

    // Call the associated function, directly from the type.
    // This function is namespaced by the struct. The :: syntax is used for both associated functions and namespaces.
    let square_1 = Rectangle::square(30);
    println!(
        "The area of the square is {} square pixels.",
        square_1.area()
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementation of methods for the Rectangle type.
// Everything within this impl block will be associated with the Rectangle type.
// If we wanted to change the instance that we've called the method on as part of what the method does, we'd use &mut self as the first parameter.
impl Rectangle {
    // area method: &self is short for self: &Self
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
    // Note that we can choose to give a method the same name as one of the struct’s fields. For example, we can define a method on Rectangle also named width:
    fn width(&self) -> bool {
        return self.width > 0;
    }
    // Add the can_hold method with other parameter
    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }

    // Let's provide an associated function named square that would have one dimension parameter and use that as both width and height, thus making it easier to create a square Rectangle rather than having to specify the same value twice.
    // The Self keywords in the return type and in the body of the function are aliases for the type that appears after the impl keyword, which in this case is Rectangle.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// Multiple impl blocks for a struct
impl Rectangle {
    fn area_2(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold_2(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
