use std::{io::{self, Write}, vec};

fn main() {
    println!("Hello, world!");

    let mut input = String::new();

    print!("Enter your operation : ");
    let _ = io::stdout().flush();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("the user input is : {}", input);

    let operation = clean_input(&mut input);

    println!("input after cleaning : {}", operation);

    let split_operation = split_operation(&operation);
    println!("The operation has been split :");
    for i in 0..split_operation.len() {
        println!("{}", split_operation[i]);
    }


    println!("10 + 3 = {}", calculate(10, 3, '+'));
    println!("10 - 3 = {}", calculate(10, 3, '-'));
    println!("10 * 3 = {}", calculate(10, 3, '*'));
    println!("10 / 3 = {}", calculate(10, 3, '/'));

}

fn clean_input(input: &mut String) -> String{
    input.replace(' ', "")
}

fn is_a_symbol(c: &char) -> bool {
    ['+', '-', '*', '/'].contains(c)
}

fn split_operation(operation: &String) -> Vec<&str> {
    let mut v: Vec<&str> = Vec::new();

    let mut last_symbol = 0;
    for (index, c) in operation.chars().enumerate() {
        if is_a_symbol(&c) || index == operation.len() - 1 {
            v.push(operation.get(last_symbol..index).expect("Error"));
            last_symbol = index;
            v.push(operation.get(last_symbol..last_symbol + 1).expect("Error"));
            last_symbol = index + 1;
        }
    }

    v
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
