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

fn to_snake_case(input: &str)->String {
    let mut snake=String::new();
    for i in input.chars(){
        if i.is_uppercase(){
            snake=format!("{}_{}",snake,i.to_lowercase());
        }else{
            snake=format!("{}{}",snake,i);
        }
    }
    snake
}
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
    println!("snake cased: {}", to_snake_case(&input));
}
