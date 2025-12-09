fn main() {

    println!("=================LOOP=================");
    let mut x = 0;
    loop {
        x += 1;
        println!("x is {}", x);
        if x == 10 {
            break;
        }
    };
    println!("=================LOOP with return value=================");
    x = 0;
    let result = loop {
        x += 1;
        println!("x is {}", x);
        if x == 10 {
            break x * x;
        }
    };
    println!("The result is {result}");

    println!("=================LOOP with continue=================");
    x = 0;
    loop {
        x += 1;
        if x == 10 {
            continue;
        }
        println!("x is {}", x);
        if x == 15 {
            break;
        }
    }

    println!("=================NESTED LOOP WITH BREAK AND CONTINUE=================");
    let mut y = 0;

}
