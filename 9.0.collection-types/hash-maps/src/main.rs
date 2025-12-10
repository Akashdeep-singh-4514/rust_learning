fn main() {
    // Hash maps are collections of key-value pairs.
    // They are useful when you want to look up a value by a specific key, quickly!

    use std::collections::HashMap;

    // Create a new, empty hash map
    let mut scores = HashMap::new();

    // Insert some key-value pairs
    scores.insert("Alice", 50);
    scores.insert("Bob", 35);

    println!("Scores: {:?}", scores);

    // You can retrieve a value by its key
    let alice_score = scores.get("Alice"); // returns an Option<&i32>
    match alice_score {
        Some(score) => println!("Alice's score is {}", score),
        None => println!("No score for Alice!"),
    }

    // Overwriting a value for an existing key
    scores.insert("Alice", 80);
    println!("Updated Scores: {:?}", scores);

    // Iterating over all key-value pairs
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    // Using non-string types as keys is allowed too!
    let mut numbers = HashMap::new();
    numbers.insert(1, "one");
    numbers.insert(2, "two");
    println!("Numbers: {:?}", numbers);

    // Important: The key type must implement Hash, Eq, and usually Copy/Clone.
    // Hash maps do not guarantee the order of elements, so keep that in mind!
}
