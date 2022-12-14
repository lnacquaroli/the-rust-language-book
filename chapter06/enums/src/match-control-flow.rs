// The match control flow construct

// Allows you to compare a value against a series of patterns and then execute code based on which pattern matches.
// Patterns can be made up of literal values, variable names, wildcards, and many other things.
// Values go through each pattern in a match, and at the first pattern the value “fits,” the value falls into the associated code block to be used during execution.
// Match can have multiple arms, composed by a pattern and an expression, separated by =>.
// Each arm is separated by a comma.
// Match is useful with multiple arms, but also for handling errors.

// Another useful feature of match arms is that they can bind to the parts of the values that match the pattern. This is how we can extract values out of enum variants.

// Combining match and enums is useful in many situations. You’ll see this pattern a lot in Rust code: match against an enum, bind a variable to the data inside, and then execute code based on it. It’s consistently a user favorite.

// We can write a function that takes an unknown coin (US) and, in a similar way as a counting machine, determines which coin it is and returns its value in cents.

// Matches are exhaustive
// The arms' patterns must cover all possibilities.
// Removing the None arm in the plus_one function below it willl cause an error in the compiler

fn main() {
    // Matching with Option<T>
    // Instead of comparing coins, we’ll compare the variants of Option<T>, but the way that the match expression works remains the same.
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // Catch all patterns and the _ placeholder
    // When you want to explicitly consider some options and the other is irrelevant
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    // Instead of other, Rust has a special catch-all pattern: _
    // This way we exhaustively consider all the options, with the important ones declared and the rest sent to the _ syntax
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// Inside a function can be used as a multiple dispatch (too Julian I guess)
fn value_in_cents(coin: Coin) -> u8 {
    return match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    };
}

// Multiple expressions in a code pattern, with {}
fn value_in_cents_mult(coin: Coin) -> u8 {
    return match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            return 1;
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    };
}

// Bind values to the parts of the values that match the pattern.

// As an example, let’s change one of our enum variants to hold data inside it. From 1999 through 2008, the United States minted quarters with different designs for each of the 50 states on one side. No other coins got state designs, so only quarters have this extra value. We can add this information to our enum by changing the Quarter variant to include a UsState value stored inside it.
#[derive(Debug)] // so we can inspect the state
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum CoinBind {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// To call this function:
// let value = value_in_cents(Coin::Quarter(UsState::Alaska))
// let coin = Coin::Quarter(UsState::Alaska)
// The binding for state will be the value UsState::Alaska.
// We can then use that binding in the println! expression,
// thus getting the inner state value out of the Coin enum variant for Quarter.
fn value_in_cents_bind(coin: CoinBind) -> u8 {
    match coin {
        CoinBind::Penny => 1,
        CoinBind::Nickel => 5,
        CoinBind::Dime => 10,
        CoinBind::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// Matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    return match x {
        None => None,
        // The i binds to the value contained in Some, so i takes the value 5.
        Some(i) => Some(i + 1),
    };
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}
