use std::io;

enum Operations {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn main() {
    println!("Hello, this is a simple calculator.");
    println!("You provide the two numbers and I do the magic.");
    println!("Very limited magic.");
    println!("Like only 4 operations. And for two numbers only. Sorry.");
    println!("But you can memorize them. Then it is infinite, right?");
    println!("Also the number has to be a float. Like 45.0 or 10.8.");
    println!("We have all kinds of limitations around here.");

    loop {
        println!("Please enter the first number: ");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to read line");
        let parsed_input1: Result<f64, _> = input1.trim().parse();
        let number1 = match parsed_input1 {
            Ok(number) => {
                println!("The first number you entered is: {}", number);
                number
            }
            Err(_) => {
                println!("Invalid input. Please enter a valid number");
                continue; 
            }
        };

        println!("Please enter the second number:");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read line");
        let parsed_input2: Result<f64,_> = input2.trim().parse();
        let number2 = match parsed_input2 {
            Ok(number) => {
                println!("The second number you entered is: {}", number);
                number
            }
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue; 
            }
        };

        println!("Please choose the operation type:");
        println!("'a' for Addition");
        println!("'s' for Subtraction");
        println!("'m' for Multiplication");
        println!("'d' for Division");

        let mut operation_choice = String::new();
        io::stdin()
            .read_line(&mut operation_choice)
            .expect("Failed to read line");

        let operator = match operation_choice.trim() {
            "a" => Operations::Add,
            "s" => Operations::Subtract,
            "m" => Operations::Multiply,
            "d" => Operations::Divide,
            _ => {
                println!("Invalid operation choice.");
                continue; 
            }
        };

        if let Some(result) = perform_operation(operator, number1, number2) {
            println!("Result: {}", result);
        } else {
            println!("Invalid operation");
        }
        loop {
            println!("Please enter 'q' to quit or 'c' to continue.");

            let mut input_2 = String::new();
            io::stdin()
                .read_line(&mut input_2)
                .expect("Failed to read line");

            let trimmed_input = input_2.trim();

            match trimmed_input {
                "q" => {
                    println!("Closing the program.");
                    return;
                }
                "c" => {
                    break;
                }
                
                _ => {
                    println!("Invalid input. Please try again.");
                    println!("(Did you enter something other than 'q' or 'c' ?)");
                }
            }
        }

    }
}

fn perform_operation(operator: Operations, number1: f64, number2: f64) -> Option<f64> {
    match operator {
        Operations::Add => Some(number1 + number2),
        Operations::Subtract => Some(number1 - number2),
        Operations::Multiply => Some(number1 * number2),
        Operations::Divide => {
            if number2 != 0.0 {
                Some(number1 / number2)
            } else {
                None
            }
        }
    }
}
