/* 
    Condotiolans in Rust
    1. if
    2. if else
    3. if else if
    4. if else if else
    5. if let
    6. match
*/

/*
This program checks if a person is an adult or a minor
based on their age. If the age is 18 or older, they are considered an adult.
Otherwise, they are considered a minor.
*/

fn main() {
    // Declare a variable 'age' of type i32 and initialize it with the value 18
    let age: i32 = 10;

    // Evaluate if the age is greater than or equal to 18
    if age >= 18 {
        // If the condition is true, print that the person is an adult
        println!("You are an adult");
    } else {
        // If the condition is false, print that the person is a minor
        println!("You are a minor");
    }
}

