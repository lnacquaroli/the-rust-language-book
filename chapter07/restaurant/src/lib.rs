// Separating modules into different files

// The same reasoning applies to binary crates, using src/main.rs instead of src/lib.rs.

// Bring the file into scope
// To make this work, we need to create the file src/front_of_house.rs

mod front_of_house;

// Bring hosting into scope
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
