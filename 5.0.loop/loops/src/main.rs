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

    // Example of a simple loop using a 'while' loop
    let mut count = 0;
    println!("=================WHILE LOOP EXAMPLE=================");
    while count < 5 {
        println!("count is {}", count);
        count += 1;
        // Here, the 'break' statement will stop the loop if count reaches 3
        if count == 3 {
            println!("Breaking out of loop when count is {}", count);
            break;
        }
    }
    println!("Loop ended at count = {}", count);

    // Example of a 'for' loop
    println!("=================FOR LOOP EXAMPLE=================");
    for num in 1..5 {
        if num == 3 {
            println!("Breaking the loop early at num = {}", num);
            break; // 'break' will immediately exit the for-loop when num is 3
        }
        println!("num is {}", num);
    }
    println!("For loop ended before reaching 5 due to break");

    // Example of nested loops and how break works with labels
    println!("=================NESTED LOOP BREAK EXAMPLE=================");
    'outer: for i in 1..4 {
        for j in 1..4 {
            if i == 2 && j == 2 {
                println!("Breaking the inner loop when i = {} and j = {}", i, j);
                break ; // This breaks out of both loops
            }
            println!("i = {}, j = {}", i, j);
        }
    }
    println!("Nested loop break with label completed.");

    println!("=================NESTED LOOP BREAK EXAMPLE with LABEL=================");
    'outer: for i in 1..4 {
        for j in 1..4 {
            if i == 2 && j == 2 {
                println!("Breaking the  outer loop when i = {} and j = {}", i, j);
                break 'outer; // This breaks out of both loops
            }
            println!("i = {}, j = {}", i, j);
        }
    }
    println!("Nested loop break with label completed.");

    // Explanation:
    // - The 'break' statement immediately ends the nearest loop that contains it.
    // - In nested loops, you can use a label (e.g., 'outer:) with break to exit an outer loop.

}
