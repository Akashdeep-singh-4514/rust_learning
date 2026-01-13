fn main() {
    // In Rust, strings are UTF-8 encoded by default.
    // This means they can contain any valid Unicode characters, not just ASCII!

    // The simplest string type is &str, called a "string slice":
    let hello = "Hello, world!"; // This is a string slice (&str)

    println!("A string slice: {}", hello);

    // The String type is a growable, heap-allocated string:
    let mut s = String::from("Hello");
    s.push_str(", 世界!"); // You can append Unicode text (here 世界 means 'world' in Chinese)
    println!("A heap-allocated String: {}", s);

    // You can use many non-ASCII characters:
    let emoji = String::from("😃🎉🦀");
    println!("Emoji string: {}", emoji);

    // You can iterate over individual Unicode scalar values (chars):
    println!("Characters in '{}':", s);
    for c in s.chars() {
        println!("{}", c);
    }

    // You can also iterate over the bytes:
    println!("Bytes in '{}':", s);
    for b in s.bytes() {
        println!("{}", b);
    }

    // Note: Because strings are UTF-8, indexing is not allowed!
    // let c = s[0]; // This won't compile

    // But you can slice as long as the indices are at valid char boundaries:
    let slice = &s[0..5]; // "Hello" is 5 bytes (all ASCII)
    println!("First 5 bytes (characters) of s: {}", slice);

    // Let's see an example with multi-byte Unicode:
    let text = "Здравствуйте"; // "Hello" in Russian
    println!("Russian text: {}", text);
    println!("Number of bytes: {}", text.len());
    println!("Number of characters: {}", text.chars().count());
    println!("First 2 characters: {}", &text[0..4]); // Each Cyrillic char takes 2 bytes
}
