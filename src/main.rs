

use std::io;

fn main() {
    // Collect the first number from the user
    let num1: f64 = get_user_input("Enter the first number:");

    // Collect the second number from the user
    let num2: f64 = get_user_input("Enter the second number:");

    // Perform operations
    let sum = num1 + num2;
    let difference = num1 - num2;
    let product = num1 * num2;

    // Check if the second number is not zero before performing division
    let quotient = if num2 != 0.0 { num1 / num2 } else { f64::NAN };

    // Display the results
    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);

    if quotient.is_nan() {
        println!("Cannot divide by zero.");
    } else {
        println!("Quotient: {}", quotient);
    }
}

// Helper function to get user input
fn get_user_input(prompt: &str) -> f64 {
    // Display the prompt to the user
    println!("{}", prompt);

    // Create a mutable String to store user input
    let mut input = String::new();

    // Read the user input from the standard input (keyboard)
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input as f64
    match input.trim().parse() {
        // If parsing succeeds (Ok), return the parsed number
        Ok(num) => num,
        // If parsing fails (Err), handle the error
        Err(_) => {
            // Display an error message to the user
            println!("Invalid input. Please enter a valid number.");
            
            // Recursive call to get a valid input
            get_user_input(prompt)
        }
    }
}
