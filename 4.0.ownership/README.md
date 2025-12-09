🔑 Ownership in Rust
Ownership is Rust's most important and unique feature. It's a set of rules the compiler checks at compile time to manage memory without a garbage collector. The goal is to provide memory safety without runtime overhead.

🌟 Three Rules of Ownership
The rules that govern how data is handled and allocated in memory are simple yet strict:

Each value in Rust has a variable that's its owner.

Analogy: Think of the variable as the sole legal guardian responsible for that data.

There can be only one owner at a time.

Effect: When a value is assigned to another variable or passed to a function, the ownership is moved from the original variable to the new one. The original variable is no longer valid.

When the owner goes out of scope, the value will be dropped.

Effect: Rust automatically calls a function called drop when the variable (owner) goes out of the curly braces ({}) of its scope. This frees the associated memory, ensuring no memory leaks.