fn main() {
    // #1 Repeating code loop
    loop{
        println!("Hello, World!");
    }


    // Returning values from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 20 {
            break counter - 100;
        }
    };
    println!("The new result is {result}");

    //#2 Loop Labels to Disambiguate Between Multiple Loops

    let mut count = 0;
    'counting_up: loop {
        println!("count: {count}");
        let mut remaining = 10;
        loop{
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
             if count == 2 {
                break 'counting_up;
             }
            remaining -= 1;
        }
        count += 1;
    }

    // #3 While Loop
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
        break;
    }
    println!("HEY!!!");

    // #4 Looping Through a Collection with for loop
    let a = [1, 2, 3, 4, 5, 6];
    let b = ["a", "b", "c", "d", "e", "f"];

    for element in a {
        println!("{element}");
    }
    for letter in b {
        println!("{letter}");
    }

}

