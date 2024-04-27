// Primitive data types
// int, float, bool, char

fn main(){
    // Integer
    // Rust has signed (+ and -)  and unsigned integer (only+) types of different sizes.
    // i8, i16, i32, i64, i128: Signed integers.
    // u8, u16, u32, u64, u128: Unsigned integers.
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);
// diff bet i32 (32 bits) and i64(64 bits)
// range : 
// i32 - 2147483647
// i64 - 9223372036854775807
    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;
    println!("Maximum value of i32: {}", e);
    println!("Maximum value of i64: {}", i);
// ==========================================
// Floats [Floating Point Types]
// f32, f64
    let pi: f64 = 3.14;
    println!("Value of pi: {}", pi);
// Boolean Values: true, false
let is_snowing: bool = true;
println!("Is it snowing ? {}", is_snowing);
// Charachter Type - char
let letter: char = 'a';
println!("First letter of the alphabet: {}", letter);
}

