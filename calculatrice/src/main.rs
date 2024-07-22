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

    let result = calculate_full_operation(&split_operation);
    println!("result : {}", result);
    // let result = calculate(&split_operation[0], &split_operation[2], &split_operation[1]);
    // println!("The result of the operation is : {} {} {} = {}", split_operation[0], split_operation[1], split_operation[2], result);
}

fn has_more_than_one_operator(v_operation: &Vec<String>) -> bool {
    //println!("v_operation.len() = {}", v_operation.len());
    v_operation.len() > 3
}

fn is_single_member(v_operation: &Vec<String>) -> bool {
    v_operation.len() == 1
}

fn calculate_full_operation(v_operation: &Vec<String>) -> i32 {
    
    let result: i32;

    if is_single_member(v_operation) {
        result = string_to_i32(&v_operation[0]);

    } else {

        let mut left_member = 0;
        let mut right_member = 0;
        let mut operator: String = String::new();
    
        if has_more_than_one_operator(v_operation) {
            //println!("More than one operator");
            println!("left _operand : {:?}", &v_operation[0..3]);
            left_member = calculate_full_operation(&v_operation[0..3].to_vec());
            operator = v_operation[3].to_string();
            println!("right_operand : {:?}", &v_operation[4..]);
            right_member = calculate_full_operation(&v_operation[4..].to_vec());
        } else {
            left_member = string_to_i32(&v_operation[0]);
            operator = v_operation[1].to_string();
            right_member = string_to_i32(&v_operation[2]);
        }
        
        result = calculate(left_member, right_member, operator)
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

    // let left: i32 = string_to_i32(left_member.to_string());
    // let right: i32 = string_to_i32(right_member.to_string());


    match operator.as_str() {
        "+" => left_member + right_member,
        "-" => left_member - right_member,
        "*" => left_member * right_member,
        "/" => left_member / right_member,
        _ => panic!("Unknown operator"),
    }

}
