fn main() {

    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    let mut result: Option<f32> = divide(10.0, 0.0);
    match result {
        Some(result) => println!("Result: {}", result),
        None => println!("Error: Division by zero"),
    }
    result = divide(10.0, 2.0);
    match result {
        Some(result) => println!("Result: {}", result),
        None => println!("Error: Division by zero"),
    }
}

fn divide(a: f32, b: f32) -> Option<f32> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}