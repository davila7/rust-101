/*
    Variable:

    Integer types: Rust has several integer types with different sizes and signedness.
    Floating-point types: For decimal numbers, Rust provides f32 and f64.
    Boolean type: Represents true or false values.
    Character type: Represents a single Unicode character.
    String types: Rust has two main string types: &str (string slice) and String (owned string).
    Array type: Fixed-size list of elements of the same type.
    Slice type: A view into a contiguous sequence, like an array.
    Tuple type: A fixed-size collection that can hold values of different types.
    Vector type: A dynamically-sized array that can grow or shrink.
    Option type: Represents an optional value, either Some(value) or None.
    Result type: Represents either success (Ok) or failure (Err).

*/

fn main() {
    // Integer types
    let a: i32 = 42;    // 32-bit signed integer
    let _b: u64 = 100;   // 64-bit unsigned integer ( _ is used to ignore the warning)
    let c: i8 = -5;     // 8-bit signed integer
    
    // Floating-point types
    let d: f32 = 3.14;  // 32-bit float
    let e: f64 = 2.71828; // 64-bit float (double precision)
    
    // Boolean type
    let f: bool = true; // Can be true or false
    
    // Character type
    let g: char = 'A';  // Single Unicode character
    
    // String types
    let h: &str = "Hello, world!"; // String slice (immutable, borrowed)
    let i: String = String::from("Hello, Rust!"); // Owned string (mutable, growable)
    
    // Array type (fixed-size)
    let j: [i32; 5] = [1, 2, 3, 4, 5]; // Array of 5 i32 elements
    
    // Slice type (dynamically-sized view into a contiguous sequence)
    let k: &[i32] = &j[1..4]; // Slice of j, from index 1 to 3
    
    // Tuple type
    let l: (i32, f64, char) = (1, 3.14, 'A'); // Tuple with different types
    
    // Vector type (dynamically-sized array)
    let mut m: Vec<i32> = vec![1, 2, 3, 4, 5]; // Vector of i32 elements
    m.push(6); // Add an element to the vector
    
    // Option type (represents optional values)
    let n: Option<i32> = Some(42);
    let o: Option<i32> = None;
    
    // Result type (represents success or failure)
    let p: Result<i32, String> = Ok(42);
    let q: Result<i32, String> = Err(String::from("Something went wrong"));

    // Printing values to see the results
    println!("Integer: {}", a);
    println!("Integer: {}", c);
    println!("Float: {}", d);
    println!("Float: {}", e);
    println!("Boolean: {}", f);
    println!("Character: {}", g);
    println!("String slice: {}", h);
    println!("Owned string: {}", i);
    println!("Array: {:?}", j);
    println!("Slice: {:?}", k);
    println!("Tuple: {:?}", l);
    println!("Vector: {:?}", m);
    println!("Option (Some): {:?}", n);
    println!("Option (None): {:?}", o);
    println!("Result (Ok): {:?}", p);
    println!("Result (Err): {:?}", q);
}
