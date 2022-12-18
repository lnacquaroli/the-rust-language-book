/*  Controlling test runs

I a test fail, the output printed can be seen running:
> cargo test -- --show-output

To filter out some of the functions, specify the one you want to test
> cargo test one_hundred

You can run multiple tests and filter some others by using a piece of the functions names:
> cargo test add
Will run the two functions that contains add in their names.

To ignore a function to test, add the #[ignore] attribute at the beginning.
After adding the attribute, you can:
1. run only the function with it as follows:
> cargo run -- --ignored
2. Run them all:
> cargo run -- --include-ignored
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    // #[test]
    // fn this_test_will_fail() {
    //     let value = prints_and_returns_10(8);
    //     assert_eq!(5, value);
    // }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    return 10;
}

pub fn add_two(a: i32) -> i32 {
    return a + 2;
}
