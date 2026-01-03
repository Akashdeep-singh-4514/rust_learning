use crate::questions::utils::inputs;

fn reverse(input: &str) -> String {
    let mut rev = String::new();
    for c in input.chars() {
        rev = format!("{}{}", c, rev);
    }
    return rev;
}

fn is_palindrome(input: &str) -> bool {
    return input == reverse(input);
}

fn to_snake_case(input: &str) {}
pub fn run() {
    let input = inputs::str_inputs().replace("\n", "");
    let rev = reverse(&input);
    let mut palindrome = String::from("no");
    if is_palindrome(&input) {
        palindrome = format!("yes");
    }
    // println!("input",input);
    println!("Reverse: {}", rev);
    println!("is palindrome: {}", palindrome);
}
