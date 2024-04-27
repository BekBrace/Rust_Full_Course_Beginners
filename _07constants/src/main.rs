// Constants
fn main() {
    println!("Hello, world!");
    let mut x = 5;
    
    const Y: i32 = 10;
    
    println!("The value of x is: {}", x);   
    println!("The value of Y is: {}", Y);
    println!("The value of PI is: {}", PI);
    println!("The value of 3 Hours in seconds is: {}", THREE_HOURS_IN_SECONDS);
}

// You can declare a constant with a type annotation.
const PI: f64 = 3.141592653;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

