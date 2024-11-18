use std::io;



fn main() {
    println!("Simple Calculator");
    println!("Enter the first number:");
    let mut first_number = String::new();

    // read the first number
    io::stdin()
        .read_line(&mut first_number)
        .expect("Failed to read line");
    
    // convert the first number to a float
    let first_number: f32 = first_number.trim().parse()
                                        .expect("Please type a number!");

    println!("Enter the second number:");
    let mut second_number = String::new();

    // read the second number
    io::stdin()
        .read_line(&mut second_number)
        .expect("Failed to read line");
    
    // convert the second number to a float
    let second_number: f32 = second_number.trim().parse()
                                        .expect("Please type a number!");

    println!("Enter the operation (+, -, *, /):");
    let mut operation = String::new();

    // read the operation
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read line");

    // remove the newline character
    let operation = operation.trim();

    // perform the operation
    let result = match operation {
        "+" => first_number + second_number,
        "-" => first_number - second_number,
        "*" => first_number * second_number,
        "/" => {
            if second_number == 0.0 {
                println!("Error: Division by zero is not allowed.");
                return;
            }
            first_number / second_number
        }
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    // print the result
    println!("The result is: {}", result);

}
