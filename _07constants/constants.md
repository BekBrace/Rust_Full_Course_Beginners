Constants
Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.

>>> First, you aren’t allowed to use mut with constants. Constants aren’t just immutable by default—they’re always immutable. 

>>> You declare constants using the const keyword instead of the let keyword, and the type of the value must be annotated.

>>> Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.
If not used >>> _XXX_ constants

The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.

Here’s an example of a constant declaration:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

The constant’s name is THREE_HOURS_IN_SECONDS and its value is set to the result of multiplying 60 (the number of seconds in a minute) by 60 (the number of minutes in an hour) by 3 (the number of hours we want to count in this program). Rust’s naming convention for constants is to use all uppercase with underscores between words. The compiler is able to evaluate a limited set of operations at compile time, which lets us choose to write out this value in a way that’s easier to understand and verify, rather than setting this constant to the value 10,800. See the Rust Reference’s section on constant evaluation for more information on what operations can be used when declaring constants.




```rust
// Constants for conversion rates
const MILES_TO_KILOMETERS: f64 = 1.60934;
const KILOGRAMS_TO_POUNDS: f64 = 2.20462;

// Convert miles to kilometers
fn miles_to_kilometers(miles: f64) -> f64 {
    miles * MILES_TO_KILOMETERS
}

// Convert kilograms to pounds
fn kilograms_to_pounds(kilograms: f64) -> f64 {
    kilograms * KILOGRAMS_TO_POUNDS
}

fn main() {
    println!("Welcome to the Unit Converter!");

    let distance_in_miles = 50.0; // Distance in miles
    let weight_in_kg = 75.0; // Weight in kilograms

    // Convert distance and weight
    let distance_in_kilometers = miles_to_kilometers(distance_in_miles);
    let weight_in_pounds = kilograms_to_pounds(weight_in_kg);

    println!("Distance {} miles is equal to {:.2} kilometers.", distance_in_miles, distance_in_kilometers);
    println!("Weight {} kilograms is equal to {:.2} pounds.", weight_in_kg, weight_in_pounds);
}
```