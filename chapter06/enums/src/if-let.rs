// Concise control flow with if let

// The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest.

fn main() {
    // Only wants to execute code if the value is the Some variant.
    let config_max = Some(3u8);
    match config_max {
        // max is bound to the value and type that config_max has with Some
        Some(max) => println!("the maximum is configured to be {}", max),
        _ => (), // Boilerplate verbosity
    }

    // Instead we can use the if let statement: one match arm only
    // max binds to the value of Some
    // Caveat: lose the exhaustive checking match enforces
    // So is useful when matching one pattern and ignores all the others.
    if let Some(max) = config_max {
        println!("the maximum is configured to be {}", max);
    }

    // if let can also include else statement
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    // Alternative
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
