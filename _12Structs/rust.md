# Defining and Instantiating Structs in Rust

Structs in Rust are used to name and package together related values, similar to tuples. 

```rust
// tuple
let rectangle = (200, 100); // width, height
```


However, unlike tuples, each field in a struct is named, providing clearer indications of what each value represents. This feature makes structs more flexible and expressive compared to tuples, where data is accessed solely by order.

```rust
struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
}
```

# Defining a Struct
To define a struct, you use the struct keyword followed by the name of the struct and the fields within curly braces. Each field is declared with a name and a type. Here's how you might define a struct to store information about a user:

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

This User struct includes fields for whether the account is active, the username, email, and a sign-in count.

# Instantiating a Struct
Creating an instance of a struct involves specifying values for each of its fields. You can do this in any order relative to the order of fields in the struct declaration:

```rust
let user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
};
```

Accessing and Modifying Struct Fields
To access a struct's fields, use dot notation. For example, user1.email retrieves the email of user1. If you declare the instance as mutable, you can also change its fields:

```rust
let mut user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
```

It's important to note that in Rust, the entire struct instance must be mutable to modify any of its fields; Rust does not allow marking only certain fields as mutable.

# Returning a Struct from a Function
You can also construct a struct instance as the last expression in a function to implicitly return it:

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email,
        username,
        sign_in_count: 1,
    }
}
```

This example uses field init shorthand, which simplifies code when variable names match struct field names.

# Creating Instances from Other Instances
Rust provides a struct update syntax to copy some or all of the values from one instance to another:

```rust
let user2 = User {
    email: String::from("another@example.com"),
    ..us`er1
};
```

Tuple Structs and Unit-Like Structs
Rust also supports tuple structs, which are like regular structs but without named fields:

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

Tuple structs are useful for simple structures where naming each field is unnecessary. On the other hand, unit-like structs have no fields and are used when you need a type to implement a trait but don't need to store data:

```rust
struct AlwaysEqual;

let subject = AlwaysEqual;
```

# Ownership and Structs
In Rust, struct fields often own their data, like the String type, which owns its contents. If you need to include references in your struct, you must use lifetimes to ensure that the data referred to by the struct is valid for the lifetime of the struct. This ensures safety and prevents data races or dangling references.

The distinctions between using owned types and references in structs reflect Rust's emphasis on memory safety and explicit lifetime management, which you will explore in more detail as you delve into the language.