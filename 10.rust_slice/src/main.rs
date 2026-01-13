fn main() {
    let input = String::from("Hello world!");
    let res: &str = find_first_word(&input);
    // cannot mutate input utill reference res is valid
    println!("for string {input} the result is {res}")
}

fn find_first_word(input: &String) -> &str {
    for (i, item) in input.chars().enumerate() {
        if item == ' ' {
            return &input[..i];
        }
        println!("{item}")
    }
    &input[..]
}
