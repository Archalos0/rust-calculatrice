pub fn check_parenthesis_number(chars: &Vec<char>) {
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


pub fn is_a_symbol(c: &char) -> bool {
    ['+', '-', '*', '/'].contains(c)
}

pub fn has_more_than_one_operator(v_operation: &Vec<String>) -> bool {
    v_operation.len() > 3
}

pub fn is_single_member(v_operation: &Vec<String>) -> bool {
    v_operation.len() == 1
}
