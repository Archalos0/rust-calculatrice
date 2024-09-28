use std::usize;

pub fn is_odd_number_parenthesis(chars: &Vec<char>) -> bool {
    let mut n_open_par = 0;
    let iter = chars.iter();
    iter.for_each(|c| if *c == '(' { n_open_par += 1 });
    
    let mut n_close_par = 0;
    let iter = chars.iter();
    iter.for_each(|c| if *c == ')' { n_close_par += 1 });

    n_open_par != n_close_par
}

pub fn is_a_symbol(c: &char) -> bool {
    ['+', '-', '*', '/'].contains(c)
}

pub fn has_more_than_one_operator(v_operation: &Vec<String>) -> bool {
    v_operation.len() > 3
}

pub fn is_single_member(v_operation: &Vec<String>) -> bool {
    v_operation.len() == 1
}

pub fn get_parenthesis_expression(expression: Vec<String>) -> Vec<usize> {
    let mut groupments: Vec<usize> = Vec::new();

    let iter = expression.iter().enumerate();

    for (index, local_exp) in iter {
        if local_exp.starts_with("(") {
            //let i32_index: i32 = index.try_into().unwrap();
            groupments.push(index);
        }
    }

    groupments
}