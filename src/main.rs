use std::io::{self, Write};
type InputNumbers = [f64; 2];

use colored::*;

fn main() {
    let commands = ["add", "subtract", "devide", "multiply", "exit"];
    let mut input_string = String::new();

    while input_string.trim() != "exit" {
        print!("{}", "Command: ".bold().blue());
        // Clear to begin
        input_string.clear();

        // Get user input
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut input_string).unwrap();
        let command = input_string.trim();

        // Check if user wants to exit
        if command == "exit" {
            return;
        }
        if !commands.contains(&command) {
            help();
        } else {
            //do some logic
            let numbers = get_numbers();
            let result = do_math(command, numbers);

            //finish it off
            println!("{}{}", "Total: ".bold().green(), result);
        }
    }
    print!("{}", "See you later!".bold().red());
}

fn help() {
    println!("");
    println!("{}", "Help".magenta().bold());
    println!("{}", "=========".magenta().bold());
    println!("add");
    println!("subtract");
    println!("multiply");
    println!("devide");
    println!("exit");
    println!("help");
    println!("");
}

fn get_numbers() -> InputNumbers {
    let mut num1_input = String::new();
    let mut num2_input = String::new();

    //gather user input
    print!("{}", "Please enter your first number: ".cyan());
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut num1_input).unwrap();

    print!("{}", "Please enter your second number: ".cyan());
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut num2_input).unwrap();

    //convert to prettier format
    let num1 = num1_input.trim().parse::<f64>().unwrap();
    let num2 = num2_input.trim().parse::<f64>().unwrap();

    return [num1, num2];
}

fn do_math(math_type: &str, numbers: InputNumbers) -> f64 {
    let mut result = 0.00;

    match math_type {
        "add" => result = numbers[0] + numbers[1],
        "subtract" => result = numbers[0] - numbers[1],
        "multiply" => result = numbers[0] * numbers[1],
        "devide" => result = numbers[0] / numbers[1],
        _ => println!("invalid command passed"),
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut result = do_math("add", [6.32, 4.10]);
        assert_eq!(result, 10.42);

        result = do_math("subtract", [5.00, 3.00]);
        assert_eq!(result, 2.00);

        result = do_math("devide", [5.00, 2.00]);
        assert_eq!(result, 2.50);

        result = do_math("multiply", [5.00, 3.00]);
        assert_eq!(result, 15.00);
    }

    #[test]
    fn it_does_not_work() {
        let mut result = do_math("add", [6.00, 4.00]);
        assert_ne!(result, 10.42);

        result = do_math("subtract", [5.00, 3.00]);
        assert_ne!(result, 3.00);

        result = do_math("devide", [5.00, 2.00]);
        assert_ne!(result, 7.50);

        result = do_math("multiply", [5.00, 3.00]);
        assert_ne!(result, 15.03);
    }
}
