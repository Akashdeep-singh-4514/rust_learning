fn num_of_uniques(nums: &Vec<i32>) -> usize {
    let mut uniques: Vec<&i32> = Vec::new();
    for i in nums {
        if !uniques.contains(&i) {
            uniques.push(i);
        }
    }
    return uniques.len();
}

#[allow(unused)]
pub fn run() {
    let nums: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let result = num_of_uniques(&nums);
    println!("Result: {}", result);
}
