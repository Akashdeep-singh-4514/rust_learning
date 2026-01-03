use std::io::{self, Write};

pub fn num_input(num_id: &str) -> i32 {
    loop {
        print!("Enter {} number: ", num_id);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let input = input.trim();

        if input.is_empty() {
            return 0;
        }

        match input.parse::<i32>() {
            Ok(num) => break num,
            Err(_) => {
                println!("❌ Invalid number, try again.");
            }
        }
    }
}

pub fn str_inputs() -> String {
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    input
}

pub fn operator_input() -> char {
    loop {
        print!("Enter operator (+ - * /): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let input = input.trim();

        if input.len() != 1 {
            println!("❌ Enter exactly one operator.");
            continue;
        }

        let op = input.chars().next().unwrap();

        if "+-*/".contains(op) {
            break op;
        } else {
            println!("❌ Invalid operator.");
        }
    }
}
