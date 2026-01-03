use rand::Rng;

use super::utils::inputs;

fn generate_random_number(max_num: i32) -> i32 {
    rand::rng().random_range(1..=max_num)
}

#[allow(unused)]
pub fn start_game() {
    let key = generate_random_number(100);
    println!("Choose a number between 1 and 100");

    loop {
        let input = inputs::num_input("your");

        match input.cmp(&key) {
            std::cmp::Ordering::Equal => {
                println!("🎉 You won!!");
                break;
            }
            std::cmp::Ordering::Greater => {
                println!("Try smaller!!");
            }
            std::cmp::Ordering::Less => {
                println!("Try bigger!!");
            }
        }
    }
}
