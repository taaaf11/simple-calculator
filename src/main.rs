use std::io;
use std::io::Write;

fn main() {
    println!("Calculator!");

    let number1 = get_number("Please enter a number: ");
    let number2 = get_number("Please enter a number: ");

    // println!("The first number is: {number1}");
    // println!("The second number is: {number2}");

    println!("Enter the operation you want to perform?");
    println!("1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");
    let choice = get_number(">>> ");

    match choice {
        1.0 => {
            let sum = add(&(number1, number2));
            println!("The sum is: {sum}");
        }
        2.0 => {
            let diff = subtract(&(number1, number2));
            println!("The differnece is: {diff}"); 
        }
        3.0 => {
            let product = multiply(&(number1, number2));
            println!("The product is: {product}")
        }
        4.0 => {
            let quotient = divide(&(number1, number2));
            match quotient {
                f64::INFINITY => {
                    println!("Nuh uh! Division by zero is not allowed :3");
                }
                value => {
                    println!("The quotient is: {value}");
                }
            }
        }
        _ => {
            println!("Unknown operation...");
        }
    }


    println!("\r\nGoodbye. Happy calculating!")
}

fn add(ns_tuple: &(f64, f64)) -> f64 {
    ns_tuple.0 + ns_tuple.1
}

fn subtract(ns_tuple: &(f64, f64)) -> f64 {
    ns_tuple.0 - ns_tuple.1
}

fn multiply(ns_tuple: &(f64, f64)) -> f64 {
    ns_tuple.0 * ns_tuple.1
}

fn divide(ns_tuple: &(f64, f64)) -> f64 {
    let quotient = ns_tuple.0 / ns_tuple.1;
    quotient
}

fn get_number(prompt: &str) -> f64 {
    let mut number = String::new();
    if prompt.len() != 0 {
        print!("{prompt}");
        io::stdout().flush();
    }
    io::stdin()
        .read_line(&mut number)
        .expect("Couldn't read the input.");

    let number = number
                    .trim()
                    .parse()
                    .expect("Please enter a number!");
    
    number
}