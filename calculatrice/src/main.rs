fn main() {
    println!("Hello, world!");

    println!("10 + 3 = {}", calculate(10, 3, '+'));
    println!("10 - 3 = {}", calculate(10, 3, '-'));
    println!("10 * 3 = {}", calculate(10, 3, '*'));
    println!("10 / 3 = {}", calculate(10, 3, '/'));
}

fn calculate(left_member: i32, right_member: i32, operator: char) -> i32{
    let mut result = 0;

    if operator == '+' {
        result = left_member + right_member;
    }

    if operator == '-' {
        result = left_member - right_member;
    }

    if operator == '*' {
        result = left_member * right_member;
    }

    if operator == '/' {
        result = left_member / right_member;
    }

    result
}
