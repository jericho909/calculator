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

    loop {
        println!("Please enter the first number:");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to read line");
        let parsed_input1: Result<i128, _> = input1.trim().parse();
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
        let parsed_input2: Result<i128,_> = input2.trim().parse();
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

        println!("Do you want to continue (c) or quit (q)?");
        let mut state = String::new();
        io::stdin().read_line(&mut state).expect("Failed to read line");

        match state.trim() {
            "c" => continue, 
            "q" => break,    
            _ => {
                println!("Invalid choice. Quitting!");
                break; 
            }
        }
    }

    println!("Thank you for using the calculator. Goodbye!");
}

fn perform_operation(operator: Operations, number1: i128, number2: i128) -> Option<i128> {
    match operator {
        Operations::Add => Some(number1 + number2),
        Operations::Subtract => Some(number1 - number2),
        Operations::Multiply => Some(number1 * number2),
        Operations::Divide => {
            if number2 != 0 {
                Some(number1 / number2)
            } else {
                None
            }
        }
    }
}
