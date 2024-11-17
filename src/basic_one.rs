use std::io;

pub fn greet() {
    println!("Hello from basic_one.rs!");
}

fn guessing_game() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

fn get_string_length(name: &str) -> u32 {
    // when we use &str we are borrowing the string, which mean we are not taking ownership of the string and we cannot change it any way

    // len counts the byte sized memory of the string
    return name.len() as u32;
}

fn get_string_length2(name: &str) -> usize {
    // it is an implicit return from a function
    name.chars().count()
}

fn is_ascii() {
    let ascii_string = "Hello, World!";
    let non_ascii_string = "Hello, 世界!";

    println!("Is '{}' ASCII? {}", ascii_string, ascii_string.is_ascii()); // true
    println!(
        "Is '{}' ASCII? {}",
        non_ascii_string,
        non_ascii_string.is_ascii()
    ); // false
}

// write a function that returns a value
// i32 is a 32 bit signed integer
// Being signed means it can accept both positive and negative numbers
// u32 is a 32 bit unsigned integer, only acceps positive numbers
fn fib(num: i32) -> i32 {
    println!("calling the function");
    let mut first = 0;
    let mut second = 1;

    if (num == 0) {
        return first;
    }

    if (num == 1) {
        return second;
    }

    // basically find the nth fibonacci number
    for i in 2..num {
        println!("i: {}", i);
        let temp = first + second;
        first = second;
        second = temp;
    }

    return second;
}

fn is_even(num: i32) -> bool {
    if (num % 2 == 0) {
        return true;
    }

    return false;
}

fn is_odd() -> bool {
    return false;
}

// write a simple function that does not return something
fn do_something() {
    println!("Doing something");
}
