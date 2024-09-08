use crate::check;

pub fn split_operation(operation: String) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();

    let mut last_index = 0;

    let mut char_v: Vec<char> = Vec::new();

    for c in operation.chars() {
        char_v.push(c);
    }

    check::check_parenthesis_number(&char_v);

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
    if last_index < operation.len() && &operation[last_index..last_index + 1] != "(" {
        let right: String = operation[last_index..].to_string();
        v.push(right.trim().to_string());
    }

    v
}

/**Adding the multiplication operator when it's not specified
 * @param operation : the vector of the chars for the operation
 * @return : the vector operation with the all the operator '*' missing
 */
fn add_missing_operator(operation: &Vec<char>) -> Vec<char> {
    let mut chars = operation.clone();

    for i in 1..chars.len() - 1 {
        if chars[i] == ')' {
            if chars[i + 1] == '(' {
                chars.insert(i + 1, '*');
            }
        }
    }

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

