use std::io;

fn main() {
    println!("Enter your name:");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Hello, {}!", name.trim());

    new_function();
    control_flow_example();
    test();
}

fn new_function() {
    let raw_input = "
        This is a raw string literal.
        It can span multiple lines and include special characters like \n and \t without needing to escape them.";
    let cleaned_input = raw_input.trim();
    println!("Raw input:\n{}", cleaned_input);
}

fn control_flow_example() {
    let number = 7;

    if number < 5 {
        println!("The number is less than 5.");
    } else if number == 5 {
        println!("The number is equal to 5.");
    } else {
        println!("The number is greater than 5.");
    }

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other number"),
    }
}
fn test() {
    let mut counter = 0;

    while counter < 5 {
        println!("Counter: {}", counter);
        counter += 1;
    }

    for i in 0..5 {
        println!("For loop iteration: {}", i);
    }
}
// am trying pushing this code to github one more time