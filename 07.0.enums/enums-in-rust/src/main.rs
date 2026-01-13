// Enums in Rust allow you to define a type which can be one of several different variants.
// Each variant can optionally have data attached to it. Enums are useful when a value can be
// one of a few possibilities.

// Here's a simple enum to represent traffic lights:
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// You can use enums in a match statement to take action based on the variant:
fn show_light_action(light: TrafficLight) {
    match light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Get Ready!"),
        TrafficLight::Green => println!("Go!"),
    }
}

// Enums can also have data attached to variants. For example:
enum Message {
    Quit,                       // no data
    Echo(String),               // tuple struct variant
    Move { x: i32, y: i32 },    // struct variant
    ChangeColor(u8, u8, u8),    // tuple variant
}

fn main() {
    // Using simple TrafficLight enum
    let light = TrafficLight::Green;
    show_light_action(light);

    // Creating enum values with data
    let msg1 = Message::Quit;
    let msg2 = Message::Echo(String::from("Hello Enums!"));
    let msg3 = Message::Move { x: 10, y: 20 };
    let msg4 = Message::ChangeColor(255, 165, 0);

    // Using match to handle different Message variants:
    let messages = [msg1, msg2, msg3, msg4];

    for msg in messages {
        match msg {
            Message::Quit => println!("Received: Quit!"),
            Message::Echo(text) => println!("Echo message: {}", text),
            Message::Move { x, y } => println!("Move to coordinates: ({}, {})", x, y),
            Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
        }
    }
}

// OUTPUT (when you run this program):
// Go!
// Received: Quit!
// Echo message: Hello Enums!
// Move to coordinates: (10, 20)
// Change color to RGB(255, 165, 0)
