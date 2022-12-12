// An example program using structs

// Let's write a program that calculates the area of a rectangle.
// We’ll start by using single variables, and then refactor the program until we’re using structs instead.

fn main() {
    let width_1 = 30;
    let height_1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width_1, height_1)
    );

    // Since the area function does not relate the arguments in the signature, we can refactor this using the tuple struct.
    let rect_1 = (30, 50);
    println!(
        "The area (tuple) of the rectangle is {} square pixels.",
        area_tuple(rect_1)
    );

    // Refactor with structs. Adds field names not to mix things up.
    let rect_2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area (struct) of the rectangle is {} square pixels.",
        area_struct(&rect_2)
    );

    // Adding Useful Functionality with Derived Traits
    // It'd be useful to print an instance of Rectangle while debugging
    // The following works because we append the trait Debug for the Rectangle struct (see below)
    println!("rect_2 = {:?}", rect_2);
    println!("rect_2 = {:#?}", rect_2);
    // Another possibility is to use the dbg! macro
    // The dbg! macro prints the stderr (and takes ownership of an expression), while the println! macro prints the stdout using a reference.
    let scale = 2;
    let rect_3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect_3);
    // We can put dbg! around the expression 30 * scale and, because dbg! returns ownership of the expression’s value, the width field will get the same value as if we didn’t have the dbg! call there. We don’t want dbg! to take ownership of rect1, so we use a reference to rect1 in the next call.
}

fn area(width: u32, height: u32) -> u32 {
    return width * height;
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1;
}

// Other attributes than derive: https://doc.rust-lang.org/reference/attributes.html
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// We want to borrow the struct rather than take ownership of it. This way, main retains its ownership and can continue using rect1, which is the reason we use the & in the function signature and where we call the function.
fn area_struct(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}
