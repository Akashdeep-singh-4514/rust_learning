fn check_valid(input: String) -> bool {
    if input.is_empty() {
        return true;
    }

    // Convert to lowercase once and collect chars
    let chars: Vec<char> = input.to_lowercase().chars().collect();
    let mut front = 0;
    let mut back = chars.len() - 1;

    while front < back {
        // Skip non-alphanumeric from front
        if !chars[front].is_alphanumeric() {
            front += 1;
            continue;
        }

        // Skip non-alphanumeric from back
        if !chars[back].is_alphanumeric() {
            back -= 1; // Should be -= not +=
            continue;
        }

        // Compare characters
        if chars[front] != chars[back] {
            return false;
        }

        front += 1;
        back -= 1; // Should be -= not +=
    }

    true
}

#[allow(unused)]
pub fn run() {
    let input = String::from("A man, a plan, a canal: Panama");
    let result = check_valid(input);
    if result {
        println!("yes it is a valid palindrom")
    } else {
        println!("no it is not a valid palindrome")
    }
}
