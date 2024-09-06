use crate::check;
use crate::parsing;

pub fn calculate(left_member: i32, right_member: i32, operator: String) -> i32 {
    match operator.as_str() {
        "+" => left_member + right_member,
        "-" => left_member - right_member,
        "*" => left_member * right_member,
        "/" => left_member / right_member,
        _ => panic!("Unknown operator"),
    }
}

pub fn calculate_expression(v_operation: Vec<String>) -> i32 {
    let mut operations = v_operation.clone();

    let result: i32;

    if check::is_single_member(&operations) {
        let new_operations = operations[0].clone();
        if new_operations.starts_with("(") {
            result = calculate_expression(parsing::split_parenthesis_expression(new_operations))
        } else {
            result = string_to_i32(&operations[0])
        }
    } else {
        if check::has_more_than_one_operator(&operations) {
            let index_operation: usize = v_operation
                .iter()
                .position(|op| ["*", "/"].contains(&op.as_str()))
                .unwrap_or(1);

            operations.insert(
                index_operation - 1,
                calculate_expression(operations[index_operation - 1..index_operation + 2].to_vec())
                    .to_string(),
            );
            operations.remove(index_operation);
            operations.remove(index_operation);
            operations.remove(index_operation);
            result = calculate_expression(operations)
        } else {
            let left_member: i32 = if operations[0].starts_with("(") {
                calculate_expression(parsing::split_parenthesis_expression(operations[0].clone()))
            } else {
                string_to_i32(&operations[0])
            };

            let operator = v_operation[1].to_string();

            let right_member: i32 = if operations[2].starts_with("(") {
                calculate_expression(parsing::split_parenthesis_expression(operations[2].clone()))
            } else {
                string_to_i32(&operations[2])
            };

            result = calculate(left_member, right_member, operator)
        }
    }

    result
}

fn string_to_i32(s: &String) -> i32 {
    let number: i32 = match s.trim().parse::<i32>() {
        Ok(number) => number,
        Err(_) => panic!("Error parsing"),
    };

    number
}