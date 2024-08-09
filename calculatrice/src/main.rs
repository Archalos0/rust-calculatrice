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

    let operation = clean_input(&mut input);

    println!("input after cleaning : {}", operation);

    let split_operation = split_operation(&operation);  

    let result = calculate_expression(&split_operation);
    println!("result : {}", result);
}

fn has_more_than_one_operator(v_operation: &Vec<String>) -> bool {
    v_operation.len() > 3
}

fn is_single_member(v_operation: &Vec<String>) -> bool {
    v_operation.len() == 1
}

fn calculate_expression(v_operation: &Vec<String>) -> i32 {
    
    let result: i32;

    if is_single_member(v_operation) {
        result = string_to_i32(&v_operation[0]);

    } else {
   
        if has_more_than_one_operator(v_operation) {
            let left_member = calculate_expression(&v_operation[0..3].to_vec());
            let operator = v_operation[3].to_string();
            let right_member = calculate_expression(&v_operation[4..].to_vec());
            result = calculate(left_member, right_member, operator)
        } else {
            let left_member = string_to_i32(&v_operation[0]);
            let operator = v_operation[1].to_string();
            let right_member = string_to_i32(&v_operation[2]);
            result = calculate(left_member, right_member, operator)
        }
        
    }

    result

}

fn clean_input(input: &mut String) -> String{
    input.replace(' ', "")
}

fn is_a_symbol(c: &char) -> bool {
    ['+', '-', '*', '/'].contains(c)
}

fn split_operation(operation: &String) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();

    let mut last_index = 0;
    for (index, c) in operation.chars().enumerate() {
        if is_a_symbol(&c) {
            let left: String = operation[last_index..index].to_string();
            v.push(left);
            v.push(c.to_string());
            last_index = index + 1;
        }
    }

    // Ajouter le dernier opérande après la boucle
    if last_index < operation.len() {
        let right: String = operation[last_index..].to_string();
        v.push(right.trim().to_string());
    }

    v
}

fn string_to_i32(s: &String) -> i32 {
    let number: i32 = match s.trim().parse::<i32>() {
        Ok(number) => number,
        Err(_) => panic!("Error parsing"),
    };

    number
}

fn calculate(left_member: i32, right_member: i32, operator: String) -> i32{

    match operator.as_str() {
        "+" => left_member + right_member,
        "-" => left_member - right_member,
        "*" => left_member * right_member,
        "/" => left_member / right_member,
        _ => panic!("Unknown operator"),
    }

}
