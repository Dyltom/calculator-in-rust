use std::io;

fn main() {
    let mut input_string = String::new();

    while input_string.trim() != "exit" {
        println!("Command: ");
        // Clear to begin
        input_string.clear();

        // Get user input
        io::stdin().read_line(&mut input_string).unwrap();

        // switch case === match around here
        match input_string.to_lowercase().trim() {
            "add" => do_math("add"),
            "subtract" => do_math("subtract"),
            "multiply" => do_math("multiply"),
            "devide" => do_math("devide"),
            _ => help(),
        }
    }
    println!("See you later!");
}

fn do_math(math_type: &str) {
    let mut num1_input = String::new();
    let mut num2_input = String::new();
    let mut result = 0.00;

    println!("Please enter your first number: ");
    io::stdin().read_line(&mut num1_input).unwrap();

    println!("Please enter your second number: ");
    io::stdin().read_line(&mut num2_input).unwrap();

    let num1 = num1_input.trim().parse::<f64>().unwrap();
    let num2 = num2_input.trim().parse::<f64>().unwrap();

    match math_type {
        "add" => result = num1 + num2,
        "subtract" => result = num1 - num2,
        "multiply" => result = num1 * num2,
        "devide" => result = num1 / num2,
        _ => println!("no match found"),
    }

    println!("Total: {}", result);
    println!("");
}

fn help() {
    println!("Help - Commands");
    println!("=========");
    println!("add");
    println!("subtract");
    println!("multiply");
    println!("devide");
    println!("exit");
    println!("help");
    println!("");
}
