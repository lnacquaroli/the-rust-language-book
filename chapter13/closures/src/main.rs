/*

Here’s the scenario: Every so often, our t-shirt company gives away an exclusive,
limited-edition shirt to someone on our mailing list as a promotion. People on the mailing
list can optionally add their favorite color to their profile. If the person chosen for a
free shirt has their favorite color set, they get that color shirt. If the person hasn’t
specified a favorite color, they get whatever color the company currently has the most of.

The way a closure captures and handles values from the environment affects which traits the
closure implements, and traits are how functions and structs can specify what kinds of
closures they can use. Closures will automatically implement one, two, or all three of
these Fn traits, in an additive fashion, depending on how the closure’s body handles the
values:

1. FnOnce applies to closures that can be called once. All closures implement at least this
trait, because all closures can be called. A closure that moves captured values out of its
body will only implement FnOnce and none of the other Fn traits, because it can only be
called once.

2. FnMut applies to closures that don’t move captured values out of their body, but that
might mutate the captured values. These closures can be called more than once.

3. Fn applies to closures that don’t move captured values out of their body and that don’t
mutate captured values, as well as closures that capture nothing from their environment.
These closures can be called more than once without mutating their environment, which is
important in cases such as calling a closure multiple times concurrently.

Functions can also implement these three traits.

*/

use std::{thread, time::Duration};

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // Storing a closure as a variable
    let _expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let _add_one_v2 = |x: u32| -> u32 { x + 1 };
    let _add_one_v3 = |x: u8| x + 1;
    let _add_one_v4 = |x: u16| x + 1;

    // The last line without .to_string will error because the compiler
    // inferred and locked the type of x as String due to the first call
    let example_closure = |x| x;
    let _s = example_closure(String::from("hello"));
    let _n = example_closure(5.to_string());

    // Closure capture immutable references (borrowing immutably)
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    // Closure with mutable references: cannot have immutable reference (println!)
    // (borrowing mutably)
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);

    // Closure taking ownership: useful to move data to another thread
    // Need to use the move keyword inside the closure signature
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // sort_by_key method from std uses the FnMut for instance
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    //
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width // returns the width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // closure expression (anonymous func): || self.most_stocked()
        return user_preference.unwrap_or_else(|| self.most_stocked());
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            return ShirtColor::Red;
        } else {
            return ShirtColor::Blue;
        }
    }
}

fn _add_one_v1(x: u32) -> u32 {
    return x + 1;
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
