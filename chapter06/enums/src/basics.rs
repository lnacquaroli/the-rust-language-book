// Enumerations or enums: basics

// Where structs give you a way of grouping together related fields and data, like a Rectangle with its width and height, enums give you a way of saying a value is one of a possible set of values.
// We may want to say that Rectangle is one of a set of possible shapes that also includes Circle and Triangle. To do this, Rust allows us to encode these possibilities as an enum.

// Say we need to work with IP addresses. Currently, two major standards are used for IP addresses: version four and version six.
// Because these are the only possibilities for an IP address that our program will come across, we can enumerate all possible variants.

// They are defined using the camel case format.
// The names (fields in structs) inside the enums are called VARIANTS.
// Unlike struct, variants are written in camel case also.

// You can put any kind of data inside an enum variant:
// strings, numeric types, or structs, and even include another enum!

// Just like with struct, we’re also able to define methods on enums.

// One important enum is Option, that comes with the standard library.
// This enum encondes the scenario in which a value could be something or nothing.
// For example, if you request the first of a list containing items, you would get a value. If you request the first item of an empty list, you would get nothing.
// Rust doesn’t have the null feature that many other languages have. Null is a value that means there is no value there. This is very error-prone.
// Rust has the Option enum to enconde this concept of null (present) or non-null (absent) states.
// Option has a large number of methods associated in many situations (docs).
// Option and its variant is couple most of the time with the match expression.

fn main() {
    // Enum values
    // The variants are namespaced under its identifier with the :: syntax.
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;
    // We can define a function that takes IpAddrKind as signature
    // and call it with whichever valid variants we want.
    // fn route(ip_kind: IpAddrKind) {}
    // route(IpAddrKind::V4)
    // route(IpAddrKind::V6)

    // We can couple enums with structs to store data as follow
    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // The same thing can be achieved more concisely using only enums
    let _home_2 = IpAddress::V4(String::from("127.0.0.1"));
    let _loopback_2 = IpAddress::V6(String::from("::1"));

    // Using different data type in enums
    let _home_3 = IpAddress2::V4(127, 0, 0, 1);
    let _loopback_3 = IpAddress2::V6(String::from("::1"));

    // Methods in enum
    let m = Message::Write(String::from("hello"));
    m.call();

    // Examples of using Option
    let _some_number = Some(5); // type Option
    let _some_char = Some('e');
    let absent_number: Option<i32> = None; // here is required to annotate the type

    // The use of Option prevents clashings like assuming something isn't null when it is.
    //let x: i8 = 5; // type i8
    //let y: Option<i8> = Some(5); // type Option<i8>
    //let sum = x + y; // error: add not implemented for these two types
    //Solution: Need to convert Option<i8> to i8 to add
}

// Define a data type with enum for the two possible IP protocols
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// We attach data to each variant of the enum directly,
// so there is no need for an extra struct.
// This way each enum variant becomes a function that takes a String.
enum IpAddress {
    V4(String),
    V6(String),
}

// another advantage to using an enum rather than a struct: each variant can have different types and amounts of associated data. Version four type IP addresses will always have four numeric components that will have values between 0 and 255. If we wanted to store V4 addresses as four u8 values but still express V6 addresses as one String value, we wouldn’t be able to with a struct. Enums handle this case with ease
enum IpAddress2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Another example
enum Message {
    Quit,                       // no associated data
    Move { x: i32, y: i32 },    // has named fields like a struct does
    Write(String),              // String
    ChangeColor(i32, i32, i32), // three i32 values
}

// The Message enum can be defined using many structs as follow
// This complicates things if we want to define a function that takes all off them unlike the enum
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// Methods inside enum
impl Message {
    fn call(&self) { // The self gets the value that we called the method on

        // method body would be defined here
    }
}

// Option enum: it comes like this in the std, inside the prelude,
// you don't have to bring it into scope. Same for its variants: None and Some.
// T indicates the type of data handle by the variant Some.
// (T is generic for: String, i32, ...)
// enum Option<T> {
//     None,
//     Some(T),
// }
