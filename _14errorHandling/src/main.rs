// Error Handling techniques [2 approaches]
// Approach 1
// enum Option<T>{ // Define the generic Option type
//     Some(T), // Represents a value
//     None, // Represents no value
// }

fn divideOption(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

    // // Approach 2
    // enum Result<T, E>{ // Define the generic Result type
    //     Ok(T), // Represents a value
    //     Err(E), // Represents an error
    // }

fn divideResult(numerator: f64, denominator: f64) -> Result<f64, String> {

        if denominator == 0.0 {
            Err("Cannot divide by 0".to_string())
        } else {
            Ok(numerator / denominator)
        }
}

fn main() {

// let result = divideOption(10.0, 2.0);
// match result{
//     Some(x) => println!("Result: {}", x),
//     None => println!("Cannot divide by Zero!"),
//     }

match divideResult(100.23, 00.00){
    Ok(result) => println!("Result: {}", result),
    Err(err) => println!("Error: {}", err),
    }
}




