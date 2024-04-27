# Rust Error Handling with Option and Result Types

# I - OPTION
Rust takes a unique and robust approach to handling errors, focusing on safety and explicitness. We'll explore the core concepts of Option and Result types, which help you manage errors predictably and securely.

# Understanding Option<T>
# The Option<T> enum is used when a value might not be present, avoiding the pitfalls of null references common in other languages.

# The Option<T> type can take one of two variants:

# Some(T): An element of type T is present.
# None: No element is present.

Here is the formal definition of Option<T>:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Example Code for Option<T>:

```rust
enum Option<T> { // Define the generic Option type
    Some(T), // Represents presence of a value
    None, // Represents absence of a value
}

fn divide(numerator: f64, denominator: f64) -> Option<f64> { // Function returns an Option
    if denominator == 0.0 { // Check if the denominator is zero
        None // Return None if true
    } else {
        Some(numerator / denominator) // Return Some with result if not zero
    }
}

fn main() {
    let result = divide(10.0, 2.0); // Use divide function
    match result { // Handle the Option using match
        Some(x) => println!("Result: {}", x), // Print result if Some
        None => println!("Cannot divide by 0"), // Print error message if None
    }
}
```

## II Result<T, E>
The Result<T, E> type is used for operations that can succeed (Ok(T)) or fail (Err(E)). This is particularly useful for more critical errors where you need to specify why an operation failed.
Here's the formal definiton of Result(<T,E>)

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
Components of the Result Enum

Ok(T): An Ok variant that holds a value of type T, which represents the successful outcome of an operation.
Err(E): An Err variant that holds a value of type E, representing the reason for failure in the operation.

# Example Code for Result<T, E>:

```rust
enum Result<T, E> { // Define the generic Result type
    Ok(T), // Represents successful outcome
    Err(E), // Represents error outcome
}

fn divide(numerator: f32, denominator: f32) -> Result<f32, String> { // Function returns a Result
    if denominator == 0.0 { // Check if the denominator is zero
        Err("Cannot divide by zero.".to_owned()) // Return Err if true
    } else {
        Ok(numerator / denominator) // Return Ok with result if not zero
    }
}

fn main() {
    match divide(10.0, 0.0) { // Use divide function
        Ok(result) => println!("Result: {}", result), // Print result if Ok
        Err(e) => println!("Error: {}", e), // Print error message if Err
    }
}
```
# III Practical Application Combining Option and Result

```rust
fn fetch_setting(key: &str) -> Option<String> { // Simulates fetching a setting value
    match key {
        "timeout" => Some("30".to_string()), // Return Some if key is "timeout"
        "retries" => Some("three".to_string()), // Return Some if key is "retries"
        _ => None, // Return None for other keys
    }
}

fn parse_setting(key: &str) -> Result<i32, String> { // Tries to parse the fetched setting
    let setting_value = fetch_setting(key); // Retrieve the setting value
    match setting_value {
        Some(value) => value.parse::<i32>().map_err(|_| format!("Invalid value for {}: {}", key, value)), // Parse and handle error
        None => Err(format!("No value found for {}", key)), // Return error if setting is not found
    }
}

fn main() {
    let keys = ["timeout", "retries", "speed"]; // Define keys to check
    for key in keys.iter() {
        match parse_setting(key) { // Process each setting
            Ok(value) => println!("The value for {} is {}", key, value), // Print if Ok
            Err(e) => println!("Error: {}", e), // Print error message if Err
        }
    }
}
```

This tutorial demonstrates how Rust's type system helps in writing safe and robust error-handling code by forcing explicit handling of both absence of values and operational failures.