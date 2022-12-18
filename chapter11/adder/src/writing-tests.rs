/*
When the assertions fail, these macros print their arguments using debug formatting, which
means the values being compared must implement the PartialEq and Debug traits.
All primitive types and most of the standard library types implement these traits.
For structs and enums that you define yourself, you’ll need to implement PartialEq to
assert equality of those types.
You’ll also need to implement Debug to print the values when the assertion fails.
Because both traits are derivable traits, this is usually as straightforward as adding the
#[derive(PartialEq, Debug)] annotation to your struct or enum definition.
*/

#[cfg(test)]
mod tests {
    // Glob (super::*) used to make the mod tests outer functions available
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        assert_eq!(result, 4); // same as result == 4
        assert_ne!(result, 0); // same as result != 0
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // The ! negates the boolean condition inside the assert, since it is false
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    #[test]
    fn greeting_contains_name_2() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            // Message added if fails
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    // Testing failure conditions
    #[test]
    // add this to test fails.
    // The expected substring should be equal to the one inside the function tested.
    // Otherwise, it will fail.
    #[should_panic(expected = "Guess value must be between 1 and 100, got 200.")]
    fn greater_than_100() {
        Guess::new(200);
    }

    // Using Result<T, E>
    // You can’t use the #[should_panic] annotation on tests that use Result<T, E>.
    // To assert that an operation returns an Err variant, don’t use the question mark
    // operator on the Result<T, E> value. Instead, use assert!(value.is_err()).
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn it_works_2() {
        let value = if 2 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        };

        assert!(value.is_err());
    }
}

pub fn add(left: usize, right: usize) -> usize {
    return left + right;
}

pub fn greeting(name: &str) -> String {
    return format!("Hello {}!", name);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        return Guess { value };
    }
}
