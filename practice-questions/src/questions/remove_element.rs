fn remove_key(nums: &Vec<i32>, key: &i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for i in nums {
        if i != key {
            result.push(*i);
        }
    }
    result
}

#[allow(unused)]
pub fn run() {
    let nums: Vec<i32> = vec![3, 2, 2, 3];
    let key: i32 = 3;
    let result = remove_key(&nums, &key);
    println!("result: {:?}", result)
}
