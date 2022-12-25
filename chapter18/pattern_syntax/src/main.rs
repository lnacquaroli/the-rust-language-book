/*

Valid syntax in patterns.

To remember:

- match expressions starts a new scope.
- match expressions stops checking arms once it has found the first matching pattern.
- With using match guards the compiler does not try to check for exhaustiveness.

- Binding with @
-- Using @ lets us test a value and save it in a variable within one pattern.

*/

fn main() {
    // Matching literals
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching named variables
    // Named variables are irrefutable patterns that match any value, and we’ve used
    // them many times in the book. However, there is a complication when you use
    // named variables in match expressions. Because match starts a new scope,
    // variables declared as part of a pattern inside the match expression will
    // shadow those with the same name outside the match construct, as is the case
    // with all variables. (To compare outter and inner scope we use match guards.)
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // y matches the value of x in the inner scope within Some(5).
        Some(y) => println!("Matched, y = {y}"),
        // Here x binds to x outside match.
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    // Multiple patterns
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching ranges
    let x = 5;

    match x {
        // If x goes from 1 to inclusively 5: 1..=5
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // Destructuring structs
    // This code creates the variables x and y that match the x and y fields of the p
    // variable. The outcome is that the variables x and y contain the values from
    // the p struct.
    let p = Point { x: 0, y: 7 };

    // x and y inside Point takes the values of those in p
    let Point { x, y } = p; // shorthand with fields == names
    assert_eq!(0, x);
    assert_eq!(7, y);

    // Destructuring literals
    // We can also destructure with literal values as part of the struct pattern
    // rather than creating variables for all the fields. Doing so allows us to test
    // some of the fields for particular values while creating variables to
    // destructure the other fields.
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // Destructuring enums
    // For some variants we can further destructure them (Move, ChangeColor, Write).
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }

    // Destructuring nested structs and enums
    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }

    // Destructuring Structs and Tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // Ignoring an entire value with _
    foo(3, 4);

    // Ignoring parts of a value with a nested _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        // In the first match arm, we don’t need to match on or use the values
        // inside either Some variant, but we do need to test for the case when
        // setting_value and new_setting_value are the Some variant.
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {setting_value:?}");

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    // Ignoring an Unused Variable by Starting Its Name with _
    let _x = 5;
    let y = 10;
    // Note that there is a subtle difference between using only _ and using a name
    // that starts with an underscore. The syntax _x still binds the value to the
    // variable, whereas _ doesn’t bind at all. Below will error:
    // let s = Some(String::from("Hello!"));
    // if let Some(_s) = s { // change _s by _ maybe
    //     println!("found a string");
    // }
    // println!("{s:?}");

    // Ignoring Remaining Parts of a Value with ..
    // The .. pattern ignores any parts of a value that we haven’t explicitly
    // matched in the rest of the pattern.
    let origin = Point3D { x: 0, y: 0, z: 0 };
    match origin {
        Point3D { x, .. } => println!("x is {x}"),
        // Otherwise: Point{x, y:_, z:_} =>
    }
    // The syntax .. can expand as needed:
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
    // Not allowed: ambiguous ..
    // match numbers {
    //     (.., second, ..) => {
    //         println!("Some numbers: {}", second)
    //     }
    // }

    // Match guard
    // It's an additional if condition specified after the pattern in a match arm.
    // The downside of this additional expressiveness is that the compiler doesn't
    // try to check for exhaustiveness when match guard expressions are involved.
    // For instance: the first arm has the pattern Some(x) and also has a match
    // guard of if x % 2 == 0 (which will be true if the number is even).
    let num = Some(4);
    match num {
        // After the matching arm we add a condition for the remainder of x
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        // The match guard if n == y is not a pattern and therefore doesn’t
        // introduce new variables. This y is the outer y rather than a new shadowed
        // y, and we can look for a value that has the same value as the outer y by
        // comparing n to y.
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    // | operator
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @ bindings
    // The at operator lets us create a variable that holds a value at the same time
    // as we’re testing that value for a pattern match.
    let msg = Message3::Hello { id: 5 };
    match msg {
        Message3::Hello {
            // With the id_variable @ we're capturing whatever value matched the
            // range while also testing that the value matched the range pattern.
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        Message3::Hello { id: 10..=12 } => {
            // The code associated with this arm doesn't have a variable that
            // contains the actual value of the id field.
            println!("Found an id in another range")
        }
        // We do have the value available to use in the arm’s code in a variable
        // named id. The reason is that we’ve used the struct field shorthand syntax.
        Message3::Hello { id } => println!("Found some other id: {id}"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),              // tuple with one element
    ChangeColor(i32, i32, i32), // tuple with 3 elements
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

// In most cases when you no longer need a particular function parameter, you would
// change the signature so it doesn’t include the unused parameter. Ignoring a
// function parameter can be especially useful in cases when, for example, you're
// implementing a trait when you need a certain type signature but the function body
// in your implementation doesn’t need one of the parameters. You then avoid getting
// a compiler warning about unused function parameters, as you would if you used a
// name instead.
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

enum Message3 {
    Hello { id: i32 },
}
