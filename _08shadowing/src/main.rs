// Shadowing
// Shadowing is not the same as marking a variable as mutable.
fn main() {
    let x = 5; // result is 5
    
    let x = x + 1; // result is 6
    
    let x = x * 1000;

    {
        let x = x * 2; // result is 12
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x in main function is: {x}");
}    

