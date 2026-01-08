fn sum_of_squares(mut n: i32) -> i32 {
    let mut sum: i32 = 0;
    while n > 0 {
        let r = n % 10;
        sum = sum + r * r;
        n = n / 10;
    }

    return sum;
}

fn is_happy(mut n: i32) -> bool {
    if n == 1 {
        return true;
    }
    if n < 10 {
        return false;
    }
    while n > 1 {
        if n < 10 {
            return false;
        }
        n = sum_of_squares(n);
    }

    return true;
}

#[allow(unused)]
pub fn run() {
    let mut num: i32 = 19;
    let result = is_happy(num);
    if result {
        println!("yes it is a happy number")
    } else {
        println!("no it is not a happy number")
    }
}
