use super::utils::inputs;

fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn subract(num1: i32, num2: i32) -> i32 {
    num1 - num2
}

fn multiply(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn divide(num1: i32, num2: i32) -> i32 {
    if num2 == 0 {
        println!("cannot divide with zero");
        return 0;
    }
    num1 / num2
}

#[allow(unused)]
pub fn start_calculator() -> i32 {
    let num1 = inputs::num_input("first");
    let opt = inputs::operator_input();
    let num2 = inputs::num_input("second");

    let result = match opt {
        '+' => add(num1, num2),
        '-' => subract(num1, num2),
        '*' => multiply(num1, num2),
        '/' => divide(num1, num2),
        _ => {
            println!("Invalid operator");
            0
        }
    };
    println!("result: {}", result);
    result
}
