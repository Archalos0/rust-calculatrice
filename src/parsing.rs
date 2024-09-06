use crate::check;

pub fn split_operation(operation: String) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();

    let mut last_index = 0;

    let mut char_v: Vec<char> = Vec::new();

    for c in operation.chars() {
        char_v.push(c);
    }

    check::check_parenthesis_number(&char_v);

    let mut reg = get_regroupments(&char_v);

    let mut char_before = ' ';

    let mut index_to_go = 0;
    for (index, c) in operation.chars().enumerate() {
        //println!("{:?}", v);
        if index_to_go >= operation.len() {
            last_index = operation.len();
            break;
        }

        if index < index_to_go {
            continue;
        }

        if c == '(' {
            if index > 0
                && char_before != ' '
                && (char_before == ')' || (char_before >= '0' && char_before <= '9'))
            {
                let left: String = operation[last_index..index].to_string();
                v.push(left);
                v.push(String::from("*"))
            }

            v.push(reg[0].clone());
            index_to_go = index + reg[0].len();
            println!("skip to : {index_to_go}");
            reg.remove(0);
        } else if check::is_a_symbol(&c) {
            if &operation[index - 1..index] != ")" {
                let left: String = operation[last_index..index].to_string();
                v.push(left);
            }
            v.push(c.to_string());
            last_index = index + 1;
        }

        char_before = c;
    }

    // Ajouter le dernier opérande après la boucle
    if last_index < operation.len() && &operation[last_index..last_index + 1] != "(" {
        let right: String = operation[last_index..].to_string();
        v.push(right.trim().to_string());
    }

    v
}

pub fn split_parenthesis_expression(s: String) -> Vec<String> {
    let mut new_operations = s;

    // removing the parenthesis
    new_operations.remove(new_operations.len() - 1);
    new_operations.remove(0);
    
    //returning the expression splitted
    split_operation(new_operations)
}

pub fn get_regroupments(chars: &Vec<char>) -> Vec<String> {
    
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

pub fn clean_input(input: &mut String) -> String {
    String::from(input.replace(' ', "").trim())
}

