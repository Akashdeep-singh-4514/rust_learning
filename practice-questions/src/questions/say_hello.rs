use super::utils::inputs;

#[allow(unused)]
pub fn hello() {
    print!("Enter your name: ");
    let mut input = inputs::str_inputs();
    let default_name = String::from("user");

    if input.len() == 0 {
        input = default_name;
    }
    println!("hello, {}", input);
}
