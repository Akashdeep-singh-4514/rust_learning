fn main() {

    // ================integer
    let int1:i32=4;
    println!("=================INTEGER=================");
    println!("integer 1 {}",int1);


    println!("=================FLOAT=================");
    // ==================float
    let float1:f32=2.0;
    println!("float 1 {}", float1);

    //-------------numerical operations

    println!("=================NUMERICAL OPERATIONS=================");
    let sum = 5 + 10;

    println!("sum of 5 and 10 is {}",sum);
    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference of 95.5 and 4.3 is {}",difference);
    // multiplication
    let product = 4 * 30;
    println!("product of 4 and 30 is {}",product);
    // division
    let quotient = 56.7 / 32.2;
    println!("quotient of 56.7 and 32.2 is {}",quotient);
    let truncated = -5 / 3; // Results in -1
    println!("truncated of -5 and 3 is {}",truncated);
    // remainder
    let remainder = 43 % 5;
    println!("remainder of 43 and 5 is {}",remainder);


}
