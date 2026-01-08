fn check_valid(input: String) -> bool {
    if input == "" {
        return true;
    }
    let mut new = String::new();
    let mut rev = String::new();
    for c in input.chars() {
        if c.is_alphanumeric() {
            new = format!("{}{}", new, c.to_lowercase());
            rev = format!("{}{}", c.to_lowercase(), rev);
        }
    }
    println!("new: {}", new);
    println!("rev: {}", rev);
    new == rev
}

#[allow(unused)]
pub fn run() {
    let input = String::from("0P");
    let result = check_valid(input);
    if result {
        println!("yes it is a valid palindrom")
    } else {
        println!("no it is not a valid palindrome")
    }
}
