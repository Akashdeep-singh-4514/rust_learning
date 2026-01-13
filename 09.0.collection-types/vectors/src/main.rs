fn main() {
    // Vectors are a growable, resizable array type provided by Rust,
    // and are part of the standard library.
    // You can think of a vector as a list that can change in size.

    // To create a new, empty vector of i32 values:
    let mut _numbers: Vec<i32> = Vec::new();

    // You can add elements to a vector using push():
    _numbers.push(10);
    _numbers.push(20);
    _numbers.push(30);

    println!("length of numbers is: {}", _numbers.len());
    println!("Numbers: {:?}", _numbers);

    // You can access elements by their index:
    println!("The first number is: {}", _numbers[0]);

    // Vectors are often created with the vec! macro for convenience:
    let _fruits = vec!["apple", "banana", "cherry"];

    println!("Fruits: {:?}", _fruits);

    // You can loop over elements in a vector:
    for fruit in &_fruits {
        println!("I like {}", fruit);
    }

    // Vectors can grow and shrink as needed!
}
