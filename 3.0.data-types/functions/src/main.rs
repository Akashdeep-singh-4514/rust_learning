fn main() {
    println!("Hello, world!");

    another_function();
    let a:i32=one();
    let b:i32=two();
    let c:i32=sum_of_two_numbers(a, b);
    print_sum_of_two_numbers(c, "sum of two numbers");
}

fn another_function() {
    println!("Another function.");
}

fn sum_of_two_numbers(a: i32, b: i32) -> i32 {
    let _c=a + b; // unused variable has to be prefixed with _
    return b+a; // return statement is required to return a value from a function
}

fn print_sum_of_two_numbers(a: i32, b: &str) {
    println!("{} : {}",b,a);
}

fn one() -> i32 {
    return 1;
}

fn two() -> i32 {
    return 2;
}