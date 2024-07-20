/*
    Pattern Matching in Rust

    This example demonstrates the advanced use of pattern matching in Rust, a powerful feature
    for controlling program flow based on the structure of data.

    Pattern matching allows comparing a value against a series of patterns and executing code
    based on which pattern matches. It's more versatile than a simple 'if-else' and can decompose
    complex values into their constituent parts.
*/

fn main() {
    // Declare a variable 'x' with the value 5
    let x = 5;

    // Use the 'match' expression to compare 'x' against various patterns
    match x {
        // Literal pattern: matches exactly with the value 1
        1 => println!("x is 1"),

        // Another literal pattern
        2 => println!("x is 2"),

        // Multiple values pattern: matches with 3 or 4
        3 | 4 => println!("x is 3 or 4"),

        // Inclusive range pattern: matches any value from 5 to 10, including both
        5..=10 => println!("x is between 5 and 10"),

        // Wildcard pattern: matches any other value not specified above
        _ => println!("x is something else"),
    }
}

/*
    Detailed explanation:

    1. The variable 'x' is initialized with the value 5.

    2. The 'match' expression compares 'x' against a series of patterns in order.

    3. Each pattern is followed by '=>' and then the code to execute if the pattern matches.

    4. Patterns are evaluated in order, and only the code for the first matching pattern is executed.

    5. Types of patterns demonstrated:
       - Literals (1 and 2): Match exactly with the specified value.
       - Multiple values (3 | 4): Match with any of the specified values.
       - Inclusive range (5..=10): Match any value in the range, including the extremes.
       - Wildcard (_): Matches any value and is typically used as a default case.

    6. In this case, since 'x' is 5, it will match the '5..=10' pattern, 
       so "x is between 5 and 10" will be printed.

    7. If 'x' had a value that didn't match any other pattern, 
       the wildcard pattern '_' would be triggered.

    Pattern matching in Rust is exhaustive, meaning all possible cases must be handled. 
    The compiler will check this to ensure no case is forgotten.
*/
