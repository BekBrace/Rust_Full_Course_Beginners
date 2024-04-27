#![allow(warnings)] 
// Structs
// Structs are used to name and package related values similar to tuples.
fn main() {
    // tuple
    let rect = (200,500);

    // Struct
    struct Book{
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }

    struct User{
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    
    let mut user1 = User{
        active: true,
        username: String::from("someusername"),
        email: String::from("someusername@m.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@m.com");
    println!("User email is {}", user1.email);

    // Return a struct from a function
    fn build_user(email: String, username: String) -> User{
        User{
            active: true,
            email,
            username,
            sign_in_count: 1,
        }
    }

    // Create instances from other instances
    let user2 = User{
        email: String::from("another@m.com"),
        ..user1
    };

    // Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let white = Color(255,255,255);

    // Unit-Like struct
    struct AlwaysEqual;
    let subject = AlwaysEqual;

}
