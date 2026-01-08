fn index_of_word(haystack: String, needle: String) -> i32 {
    let mut index: usize = 0;
    let str_size: usize = haystack.len();
    let needle_size: usize = needle.len();
    if needle_size > str_size {
        return -1;
    }
    while index < (str_size - needle_size + 1) {
        let slice = &haystack[index..index + needle_size];
        if slice == needle {
            return index as i32;
        }
        index += 1;
    }
    if index < (str_size - needle_size + 1) {
        return -1;
    }
    index as i32
}

#[allow(unused)]
pub fn run() {
    let result = index_of_word("sadbutsad".to_string(), "sad".to_string());
    println!("result: {}", result)
}
