use crate::check;
use crate::errors::CalculationError;
use crate::parsing;

pub fn launch_calcul(input: String) -> Result<i32, CalculationError> {
    let mut operation_string = input.clone();

    let operation = parsing::clean_input(&mut operation_string);
    let split_operation = parsing::split_operation(operation.clone())?;

    Ok(calculate_expression(split_operation)?)
}


pub fn calculate(left_member: i32, right_member: i32, operator: String) -> Result<i32, CalculationError> {
    match operator.as_str() {
        "+" => Ok(left_member + right_member),
        "-" => Ok(left_member - right_member),
        "*" => Ok(left_member * right_member),
        "/" => {
            if right_member == 0 {
                return Err(CalculationError::DivisionByZero);
            }
            Ok(left_member / right_member)
        },
        _ => Err(CalculationError::UnknownOperator(operator)),
    }
}

pub fn calculate_expression(v_operation: Vec<String>) -> Result<i32, CalculationError> {
    let mut operations = v_operation.clone();

    let result: i32;

    let groupments_index: Vec<usize> = check::get_parenthesis_expression(operations.clone());

    for i in 0..groupments_index.len() {
        let groupment_split = split_parenthesis_operation(operations[groupments_index[i]].clone())?;

        let groupment_calculate = calculate_expression(groupment_split)?;

        operations.insert(
            groupments_index[i],
            groupment_calculate
                .to_string(),
        );

        operations.remove(groupments_index[i] + 1);
    }

    //Si on a qu'un membre on le retourne en int
    if check::is_single_member(&operations) {
        result = string_to_i32(&operations[0])
    } else {
        if !check::has_more_than_one_operator(&v_operation) {
            result = calculate(
                string_to_i32(&operations[0]),
                string_to_i32(&operations[2]),
                operations[1].to_string(),
            )?
        } else {
            //Calcul with operation priority
            let index_operation: usize = operations
                .iter()
                .position(|op| ["*", "/"].contains(&op.as_str()))
                .unwrap_or(1);

            operations.insert(
                index_operation - 1,
                calculate_expression(operations[index_operation - 1..index_operation + 2].to_vec())?
                    .to_string(),
            );
            operations.remove(index_operation);
            operations.remove(index_operation);
            operations.remove(index_operation);

            result = calculate_expression(operations)?;
        }
    }

    Ok(result)
}

fn split_parenthesis_operation(groupment: String) -> Result<Vec<String>, CalculationError> {
    let mut groupment = groupment;

    //Remove border parenthesis
    groupment.remove(0);
    groupment.remove(groupment.len() - 1);

    let groupment_split = parsing::split_operation(groupment.clone())?;

    Ok(groupment_split)
}

fn string_to_i32(s: &String) -> i32 {
    let number: i32 = match s.trim().parse::<i32>() {
        Ok(number) => number,
        Err(_) => panic!("Error parsing"),
    };

    number
}
