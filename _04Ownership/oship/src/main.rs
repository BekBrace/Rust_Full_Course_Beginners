// Ownership, Borrowing and References

// Ownership
// ---------
// C, C++ -> Memory Managment Control Issue
// Garbage Collector solved this issue, but created a new issue -> Slow Performance:
// [stopping/Resuming the program]
// OWNERSHIP introduced by Rust to solve memory safety issues and high performance at the same time.
// What is Ownership ?
// Every value has a single owner [every variable has one value, and it is its sole owner].

// Ownership Rules
// -------------------
// 1- Each value in Rust has a variable that's its owner.
// 2- There can be only one owner at a time.
// 3- When the owner goes out of scope, the value will be dropped. 

// Example:Each value in Rust has a variable that's its owner.
// fn main(){
//     let s1 = String::from("RUST");
//     let len = calculate_length(&s1);
//     println!("Length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize{
//     s.len()
// }

// 2- There can be only one owner at a time.
// fn main(){
//     let s1 = String::from("RUST");
//     let s2 = s1;

//     // println!("{}", s1);
//     println!("{}", s2);
// }

// 3- When the owner goes out of scope, the value will be dropped.

fn main(){
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("Length of '{}' is {}.", s1, len);
}// s1 goes out of scope and its value will be dropped

fn printLost(s: &string){
    println!("{}", &s1);
}

fn calculate_length(s: &String) -> usize{
    s.len()
}
