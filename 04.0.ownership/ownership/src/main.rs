fn main() {
    // create a new string usig String::from
    let s = String::from("hello");
    let length = calculate_length(&s);
    println!("The length of '{}' is {}.", s, length);
    // s2 takes ownership of s and  s is no longer valid
    let s2 = s;
    let length2 = calculate_length(&s2);
    println!("The length of '{}' is {}.", s2, length2);
}

fn calculate_length(some_string: &String) -> usize {
    return some_string.len();
}
