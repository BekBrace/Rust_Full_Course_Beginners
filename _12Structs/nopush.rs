// Weird that in Rust, we cannot push or append to a tuple.

fn main(){
    let t = (12, 34,"adidas", false);

    let new_t = "new string";

    let extended_t = (t.0, t.1, t.2, &new_t);
     
     println!("Extended T: {:?}", extended_t);
}









