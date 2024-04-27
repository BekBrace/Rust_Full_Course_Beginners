# Storing Lists of Values with Vectors
In today's tutorial, we're going to dive into one of the fundamental collection types in Rust—the Vector, or Vec<T>. 

If you've ever needed to store a list of items in a single structure, then vectors are your go-to solution in Rust. Let’s get started!"

"Vectors in Rust are incredibly powerful. They allow you to store more than one value in a single data structure, which organizes all the values next to each other in memory. However, remember, all values in a vector must be of the same type."

"To kick things off, let’s see how to create a new, empty vector. It's pretty straightforward with the Vec::new function."


```rust

let v: Vec<i32> = Vec::new();
```

"Notice the type annotation here. Since we’re not inserting any values yet, Rust doesn’t know what type of elements we intend to store, unless we tell it explicitly. This is a key point because vectors in Rust are implemented using generics."

"More often, you'll start with some values in your vector. Rust provides a handy macro for this, called vec!, which lets us create a new vector with initial values."

```rust

let v = vec![1, 2, 3];
```

"Because we provided initial values of type i32, Rust can automatically infer that v is a vector of i32s—no need for type annotations here!"

Section: Updating a Vector

"Let’s say you want to add elements to your vector. You can do this using the push method, but remember, your vector must be mutable."

```rust
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);
v.push(8);
```
"With each push, we’re adding new elements to our vector. Pretty cool, right?"


# Section: Reading Elements of Vectors
```rust
let v = vec![1, 2, 3, 4, 5];
let third: &i32 = &v[2]; // Direct indexing
println!("The third element is {third}");

let third = v.get(2); // Using get method
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
}
```
