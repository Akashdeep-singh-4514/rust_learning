fn is_opening(c: char) -> bool {
    matches!(c, '(' | '[' | '{')
}

fn is_matched(open: char, close: char) -> bool {
    matches!(
        (open, close),
        ('(', ')') | ('[', ']') | ('{', '}')
    )
}

fn check_paranthesis(input:&str)->bool{
    let mut stack: Vec<char> = Vec::new();
    for c in input.chars(){
        if is_opening(c) {
            stack.push(c);
        } else {
            // closing bracket
            match stack.pop() {
                Some(open) if is_matched(open, c) => {}
                _ => return false, // empty stack or mismatch
            }
        }
    }
    stack.is_empty()
}

pub fn run() {
    let test_string = "([)]";
    let result = check_paranthesis(&test_string);
    println!("Result {}",result);
}

