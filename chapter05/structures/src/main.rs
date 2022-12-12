// Structures

// A struct, or structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group. If you’re familiar with an object-oriented language, a struct is like an object’s data attributes.

// The pieces of a structure can be different types.
// Unlike with tuples, you name each piece of data.
// You can access to the pieces of an instance of a struct through their names.
// These names are called fields.
// In each field we must specify the types.
// Structs are named using the camel case format.
// The instance of a struct can be mutable when binding it to a variable. (flexible)

// Rust also supports structs that look similar to tuples, called tuple structs. Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields. Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples, and when naming each field as in a regular struct would be verbose or redundant.
// Tuple structs are accessed as the normal tuples, with the . plus index notation.

// Unit-Like structs
// Structs defined without any field.
// Useful to implement a trait on some type but don't have any data to store in the type itself.

// It’s also possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes, a Rust feature that we’ll discuss in Chapter 10. Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.

fn main() {
    // Instantiation of the struct User, use the field names in any order.
    let _user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // A mutable structs can reassign values to its fields.
    let mut user1_mut = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1_mut.email = String::from("anotheremail@example.com");

    // Construct a struct from a function
    let _user1_func = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    // With shorthand
    let _user1_func_alt = build_user_alt(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    // Creating instances from other instances with struct update syntax
    // It’s often useful to create a new instance of a struct that includes most of the values from another instance, but changes some. You can do this using struct update syntax.
    let _user2 = User {
        active: _user1.active,
        username: _user1.username,
        email: String::from("another@example.com"),
        sign_in_count: _user1.sign_in_count,
    };
    // Another way to achieve this is with the .. notation
    let _user0 = build_user_alt(
        String::from("one@example.com"),
        String::from("anotherusername"),
    ); // Can't use the username field with .. since we moved it above, so need to create a new one. Notice that the types of sign_in_count and active implements the Copy trait though
    let _user2_b = User {
        email: String::from("another@example.com"),
        .._user0
    };

    // Using Tuple Structs without Named Fields to Create Different Types
    let _black = Color(0, 0, 0);
    let _origin = Point(0.0, 0.0, 0.0);

    println!(
        "Origin: (x1, x2, x3) = ({}, {}, {})",
        _origin.0, _origin.1, _origin.2
    );

    // Unit-Like Structs Without Any Fields
    let _subject = AlwaysEqual;
    print_type_of(&_subject);
}

// Example
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// Using the same name of the fields as variables simplifies the syntax.
// This is called field init shorthand syntax:
fn build_user_alt(email: String, username: String) -> User {
    return User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    };
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(f64, f64, f64);

// Unit-Like structs
struct AlwaysEqual;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
