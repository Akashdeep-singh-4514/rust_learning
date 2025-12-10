fn main() {
    //Result<T, E> = Ok(T) | Err(E)
    let mut result: Result<f32, String> = divide(10.0, 0.0);
    match result {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }
    result = divide(10.0, 2.0);
    match result {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }
}

fn divide(a: f32, b: f32) -> Result<f32, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}