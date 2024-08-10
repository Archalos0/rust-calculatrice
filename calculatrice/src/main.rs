use std::{io::{self, Write}, time::Instant};

// TODO : handle ()()

fn main() {
    let mut input = String::new();

    print!("Enter your operation : ");
    let _ = io::stdout().flush();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let time = Instant::now();

    let operation = clean_input(&mut input);

    let split_operation = split_operation(operation.clone());  

    let result = calculate_expression(split_operation.clone());

    println!("The operation and the result are :\n{} = {}\nIn : {:?}", operation.trim_end(), result, time.elapsed());
}

fn has_more_than_one_operator(v_operation: &Vec<String>) -> bool {
    v_operation.len() > 3
}

fn is_single_member(v_operation: &Vec<String>) -> bool {
    v_operation.len() == 1
}

fn split_parenthesis_expression(s: String) -> Vec<String> {
    let mut new_operations = s;

    // removing the parenthesis
    new_operations.remove(new_operations.len() - 1);
    new_operations.remove(0);
    
    //returning the expression splitted
    split_operation(new_operations)
}

fn calculate_expression(v_operation: Vec<String>) -> i32 {

    let mut operations = v_operation.clone();

    let result: i32;

    if is_single_member(&operations) {
        let new_operations = operations[0].clone();
        if new_operations.starts_with("(") {            
            result = calculate_expression(split_parenthesis_expression(new_operations))
        } else {
            result = string_to_i32(&operations[0])
        }

    } else {

        if has_more_than_one_operator(&operations) {
            
            let index_operation: usize = v_operation.iter().position(|op| ["*","/"].contains(&op.as_str())).unwrap_or(1);
            
            operations.insert(index_operation - 1, calculate_expression(operations[index_operation-1..index_operation+2].to_vec()).to_string());
            operations.remove(index_operation);
            operations.remove(index_operation);
            operations.remove(index_operation);
            result = calculate_expression(operations)
        
        } else {
            
            let left_member: i32 = if operations[0].starts_with("(") {
                calculate_expression(split_parenthesis_expression(operations[0].clone()))
            } else {
                string_to_i32(&operations[0])
            };
            
            let operator = v_operation[1].to_string();

            let right_member: i32 = if operations[2].starts_with("(") {
                calculate_expression(split_parenthesis_expression(operations[2].clone()))
            } else {
                string_to_i32(&operations[2])
            };

            result = calculate(left_member, right_member, operator)
        }
        
    }

    result

}

fn clean_input(input: &mut String) -> String{
    String::from(input.replace(' ', "").trim())
}

fn is_a_symbol(c: &char) -> bool {
    ['+', '-', '*', '/'].contains(c)
}

fn check_parenthesis_number(chars: &Vec<char>) {
    let mut n_open_par = 0;
    let iter = chars.iter();
    iter.for_each(|c| if *c == '(' { n_open_par += 1 });
    
    let mut n_close_par = 0;
    let iter = chars.iter();
    iter.for_each(|c| if *c == ')' { n_close_par += 1 });

    if n_open_par != n_close_par {
        panic!("Expression not valid");
    }
}

fn get_regroupments(chars: &Vec<char>) -> Vec<String> {
    
    let mut regroupments: Vec<String> = Vec::new();

    let iter = chars.iter().enumerate();

    let mut parenthesis_calculator = 0;
    let mut index_reg_begin = 0;
    for (index, c) in iter {

        match c {
            '(' => {
                if parenthesis_calculator == 0 {
                    index_reg_begin = index;
                }
                parenthesis_calculator += 1
            },
            ')' => {
                parenthesis_calculator -= 1;
                if parenthesis_calculator == 0 {
                    regroupments.push(chars[index_reg_begin..index+1].iter().collect());
                }
            },
            _ => continue,
        }
    }

    regroupments

}

fn split_operation(operation: String) -> Vec<String> {

    let mut v: Vec<String> = Vec::new();
    
    let mut last_index = 0;
    
    let mut char_v: Vec<char> = Vec::new();
    
    for c in operation.chars() {
        char_v.push(c);
    }

    check_parenthesis_number(&char_v);

    let mut reg = get_regroupments(&char_v);

    let mut char_before = ' ';

    let mut index_to_go = 0;
    for (index, c) in operation.chars().enumerate() {
        println!("{:?}", v);
        if index_to_go >= operation.len() {
            last_index = operation.len();
            break;
        }

        if index < index_to_go {
            continue;
        }

        if c == '(' {

            if index > 0 && char_before != ' ' && (char_before == ')' || (char_before >= '0' && char_before <= '9') ) {
                let left: String = operation[last_index..index].to_string();
                v.push(left);
                v.push(String::from("*"))
            }

            v.push(reg[0].clone());
            index_to_go = index + reg[0].len();
            println!("skip to : {index_to_go}");
            reg.remove(0);

        } else if is_a_symbol(&c) {
            if &operation[index-1..index] != ")" {
                let left: String = operation[last_index..index].to_string();
                v.push(left);
            }
            v.push(c.to_string());
            last_index = index + 1;
        }

        char_before = c;
    }

    // Ajouter le dernier opérande après la boucle
    if last_index < operation.len() && &operation[last_index..last_index+1] != "(" {
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
