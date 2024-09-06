use std::{io::{self, Write}, time::Instant};

mod calcul;
mod parsing;
mod check;

// TODO : handle ()()

fn main() {
    let mut input = String::new();

    print!("Enter your operation : ");
    let _ = io::stdout().flush();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let time = Instant::now();

    let operation = parsing::clean_input(&mut input);

    let split_operation = parsing::split_operation(operation.clone());  

    let result = calcul::calculate_expression(split_operation.clone());

    println!("The operation and the result are :\n{} = {}\nIn : {:?}", operation.trim_end(), result, time.elapsed());
}