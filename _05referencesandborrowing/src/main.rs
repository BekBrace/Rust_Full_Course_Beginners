// References and Borrowing
// Safety and Performance
// Borrowing and references are powerful concepts 

// Understanding References
// References: Enable you to borrow values without taking ownership.
// Immutable Reference.
// Mutable Reference.
// Create Reference by add "&"
// -I- Immutable Reference
// fn main() {
//     let _x = 5;
//     let _r = &_x;

//     println!("Value of _x : {}", _x);
//     println!("Value of _r : {}", _r);
// }

// -II- Mutable Reference
// fn main() {
//     let mut _x = 5;
    
//     let _r = &mut _x;
    
//     *_r += 1;
//     *_r -= 3;
    
//     println!("Value of _x : {}", _x);
//     // One very important rule of reference in Rust: one mutable reference or many immutable references.
//     // println!("Value of _r : {}", _r);
// }

// Demonstration on one mutable reference or many immutable references
// fn main(){
//     let mut account = BankAccount{
//         owner: "Alice".to_string(),
//         balance:150.55,
//     };
    
//     // Immutable borrow to check the balance
//     account.check_balance();

//     // Mutable borrow to withdraw money
//     account.withdraw(45.5);

//     // Immutable borrow to check the balance
//     account.check_balance();
    
// }

// struct BankAccount {
//     owner: String,
//     balance: f64,
// }

// impl BankAccount{
//     fn withdraw(&mut self, amount: f64){
//         println!("Withdrawing {} from account owned by {}", amount, self.owner);
//         self.balance -= amount;
//     }

//     fn check_balance(&self){
//         println!("Account owned by {} has a balance of {}", self.owner, self.balance);
//     }
// }

// III- Preventing dangling references
// fn main(){
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String{
//     let s = String::from("Hello");
//     &s
// }

// Borrowing: Use values without taking ownership
fn main(){
    // immutable borrowing
//     let s1 = String::from("hello");
//     let len = calc_len(&s1);
//     println!("The lenght of {} is {}.", s1, len);
// }
// fn calc_len(s:&String) -> usize{
//     s.len()

// Mutable Borrowing:
let mut s = String::from("RUSTY RUST");

change_string(&mut s);

println!("{}", s);

}

fn change_string(s: &mut String){
    s.push_str(", A NEW STRING!");
}





