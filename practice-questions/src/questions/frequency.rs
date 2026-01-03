use crate::questions::utils::inputs;
use std::collections::HashMap;

#[allow(unused)]
pub fn run() {
    let input = inputs::str_inputs().replace("\n", "");
    let words: Vec<&str> = input.split_whitespace().collect();
    let mut chars = String::from("");
    let mut freq_map: HashMap<&str, i32> = HashMap::new();
    for i in &words {
        chars += i;
        *freq_map.entry(i).or_insert(0) += 1;
    }
    println!("words: {}", words.len());
    println!("chars: {}", chars.len());
    println!("freq: {:?}", &freq_map);
}
