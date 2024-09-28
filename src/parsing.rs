use crate::{check, errors::CalculationError};

pub fn split_operation(operation: String) -> Result<Vec<String>, CalculationError> {
    let mut v: Vec<String> = Vec::new();

    let mut last_index = 0;

    let mut char_v: Vec<char> = Vec::new();

    for c in operation.chars() {
        char_v.push(c);
    }

    if check::is_odd_number_parenthesis(&char_v) {
        return Err(CalculationError::ErrorNumberParenthesis);
    }

    char_v = add_missing_operator(&char_v);

    let mut groupments = get_regroupments(&char_v);

    let mut index_to_go = 0;
    for (index, c) in char_v.iter().enumerate() {
        if index_to_go >= char_v.len() {
            last_index = char_v.len();
            break;
        }

        if index < index_to_go {
            last_index = index;
            continue;
        }

        if *c == '(' {
            //adding groupment
            v.push(groupments[0].clone());
            index_to_go = index + groupments[0].len();
            groupments.remove(0);
        } else if check::is_a_symbol(&c) {
            if char_v[index - 1] != ')' {
                //adding element before the operato if that's not a groupment
                let left: String = char_v[last_index..index].into_iter().collect();
                v.push(left);
            }

            //adding the operator
            v.push(c.to_string());
            last_index = index + 1;
        }
    }

    // Ajouter le dernier opérande après la boucle
    if last_index < char_v.len() && &char_v[last_index..].into_iter().collect::<String>() != "(" {
        let right: String = char_v[last_index..].into_iter().collect();
        v.push(right.trim().to_string());
    }

    Ok(v)
}

/**Adding the multiplication operator when it's not specified
 * @param operation : the vector of the chars for the operation
 * @return : the vector operation with the all the operator '*' missing
 */
fn add_missing_operator(operation: &Vec<char>) -> Vec<char> {
    let mut chars = Vec::new();

    let iter = operation.iter().enumerate();

    for (index, c) in iter {
        match *c {
            ')' => {
                chars.push(*c);
                if index < operation.len() - 1 && ( operation[index + 1] == '(' || ( operation[index + 1] >= '1' && operation[index + 1] <= '9' ) ) {
                    chars.push('*');
                }    
            },

            '(' => {
                if index > 0 && ( operation[index - 1] >= '1' && operation[index - 1] <= '9' ) {
                    chars.push('*');
                }
                chars.push(*c);
            },

            _ => { 
                chars.push(*c); 
            }
        }
    }

    // for i in 1..chars.len() - 1 {
    //     if chars[i] == ')' {
    //         if chars[i + 1] == '(' || ( chars[i+1] >= '1' && chars[i+1] <= '9' ) {
    //             chars.insert(i + 1, '*');
    //         }
    //     }

    //     if chars[i] == '(' {
    //         if chars[i - 1] == ')' ||  ( chars[i-1] >= '1' && chars[i-1] <= '9' ) {
    //             chars.insert(i, '*');
    //         }
    //     }
    // }

    chars
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

