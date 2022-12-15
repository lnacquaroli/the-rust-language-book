// Paths to an item in the module tree

// Everthing is private inside the parent module, called crate (root)
// Need to explicitly make every method, struc, enum, etc public to have acces

// We can also use pub to designate structs and enums as public, but there are a few details extra to the usage of pub with structs and enums.

// If we use pub before a struct definition, we make the struct public, but the struct’s fields will still be private. We can make each field public or not on a case-by-case basis.

// In contrast, if we make an enum public, all of its variants are then public. We only need the pub before the enum keyword.

// Employing the ue keyword we can simplify the process of writing the paths to call functions every time

// Note that use only creates the shortcut for the particular scope in which the use occurs.

// Specifying the parent module when calling the function makes it clear that the function isn’t locally defined while still minimizing repetition of the full path.
// Exceptions when bringing items with the same name, as with std::io::Results and std::fmt::Result. In this case, you do std::io and std::fmt separately.

// Another way to overcome the types with same name into scope is to use the as keyword
// use std::fmt::Result;
// use std::io::Result as IoResult; // too Pythonic
// fn funtion1() -> Result {}
// fn funtion2() -> IoResult<()> {}

// When we bring a name into scope with the use keyword, the name available in the new scope is private. To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope, we can combine pub and use. This technique is called re-exporting because we’re bringing an item into scope but also making that item available for others to bring into their scope.

mod front_of_house {
    // Need to use pub to make it public outside of the module
    pub mod hosting {
        // Need to use pub to make it public outside of the module
        pub fn add_to_waitlist() {}
    }
}

// Specifying the parent module when calling the function makes it clear that the function isn’t locally defined while still minimizing repetition of the full path.
// Exceptions when bringing items with the same name, as with std::io::Results and std::fmt::Result. In this case, you do std::io and std::fmt separately.
// Idiomatic way of bringing a function into scope
use crate::front_of_house::hosting;
// use crate::front_of_house::hosting::add_to_waitlist; // non-idiomatic, not recommended

// We can reexport the module hosting for others after bring it to scope, with pub use:
// pub use crate::front_of_house::hosting;
// Before this change, external code would have to call the add_to_waitlist function by using the path restaurant::front_of_house::hosting::add_to_waitlist(). Now that this pub use has re-exported the hosting module from the root module, external code can now use the path restaurant::hosting::add_to_waitlist() instead.

// Using external packages
// Need to specify the package name and version, and running cargo run will download it from the crates.io and install it for use.

// Using nested paths with use
// This is useful to bring multiple items into scope from the same package
//  use std::cmp::Ordering;
//  use std::io;
// could be
//  use std::{cmp::Ordering, io};
// Or
//  use std::io;
//  use std::io::Write;
// becomes
// use std::io::{self, Write};

// Glob operator
// use std::collections::*
// Brings all public items inside collections into scope
// Not recommended (except in tests) because is hard to tell what names are into scope

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Using use
    hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Structs with selective pub fields
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    // Enums with pub make all variantes available
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// Using the super syntax
fn deliver_order() {}

// Using super allows us to reference an item that we know is in the parent module, which can make rearranging the module tree easier when the module is closely related to the parent, but the parent might be moved elsewhere in the module tree someday.
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // Not public
    }

    // Also, note that because back_of_house::Breakfast has a private field, the struct needs to provide a public associated function that constructs an instance of Breakfast (we’ve named it summer here).
    // NOTE: If Breakfast didn’t have such a function, we couldn’t create an instance of Breakfast in eat_at_restaurant because we couldn’t set the value of the private seasonal_fruit field in eat_at_restaurant.
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// Need to use super to call the available hosting module
// We can also solve this problem by bringing the use crate::front_of_house::hosting inside the module customer
mod customer {

    // Use use without super
    // use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        super::hosting::add_to_waitlist();
    }
}
