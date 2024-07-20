use std::io::{self, Write};

fn main() {
    println!("Hello, world!");

    let mut input = String::new();

    print!("Enter your operation : ");
    let _ = io::stdout().flush();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("the user input is : {}", input);


    println!("10 + 3 = {}", calculate(10, 3, '+'));
    println!("10 - 3 = {}", calculate(10, 3, '-'));
    println!("10 * 3 = {}", calculate(10, 3, '*'));
    println!("10 / 3 = {}", calculate(10, 3, '/'));
}

fn calculate(left_member: i32, right_member: i32, operator: char) -> i32{
    let mut result = 0;

    if operator == '+' {
        result = left_member + right_member;
    }

    if operator == '-' {
        result = left_member - right_member;
    }

    if operator == '*' {
        result = left_member * right_member;
    }

    if operator == '/' {
        result = left_member / right_member;
    }

    result
}
