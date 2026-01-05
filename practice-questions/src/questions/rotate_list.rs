fn rotate_list(nums: &Vec<i32>) -> Vec<&i32> {
    let mut rotated: Vec<&i32> = Vec::new();
    rotated.push(&nums.last().unwrap());
    let mut i = 0;
    while i < nums.len() - 1 {
        rotated.push(&nums[i]);
        i += 1;
    }

    rotated
}

#[allow(unused)]
pub fn run() {
    let nums: Vec<i32> = vec![1, 2, 3, 4];
    let result = rotate_list(&nums);
    println!("result: {:?}", result)
}
