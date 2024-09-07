use std::{io::{self, Write}, time::Instant};

use calcul::launch_calcul;

mod calcul;
mod parsing;
mod check;
mod tests;

// TODO : handle ()()

fn main() {
    let mut input = String::new();

    print!("Enter your operation : ");
    let _ = io::stdout().flush();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let time = Instant::now();

    let result = launch_calcul(input.clone());

    println!("The operation and the result are :\n{} = {}\nIn : {:?}", input.clone().trim_end(), result, time.elapsed());
}