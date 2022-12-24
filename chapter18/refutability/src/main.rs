/*

- Patterns come in two forms: refutable and irrefutable.
- Patterns that will match for any possible value passed are irrefutable. An example
would be x in the statement let x = 5; because x matches anything and therefore
cannot fail to match.
- Patterns that can fail to match for some possible value are refutable. An example
would be Some(x) in the expression if let Some(x) = a_value because if the value in
the a_value variable is None rather than Some, the Some(x) pattern will not match.


*/

fn main() {
    // If some_option_value was a None value, it would fail to match the pattern Some
    // (x), meaning the pattern is refutable. However, the let statement can only
    // accept an irrefutable pattern because there is nothing valid the code can do
    // with a None value.
    let some_option_value: Option<i32> = None;
    //let Some(x) = some_option_value; // error, must be irrefutable with let

    // If we have a refutable pattern where an irrefutable pattern is needed, we can
    // fix it by changing the code that uses the pattern:
    if let Some(x) = some_option_value {
        println!("{}", x);
    }
    // Warning from compiler, use let x = 5...
    if let x = 5 {
        println!("{}", x);
    };
}
