pub fn sum(nums: Vec<i32>) -> i32 {
    let mut total = 0;
    for num in nums {
        total = total + num;
    }
    total
}

pub fn fill(i: u32, n: usize) -> Vec<u32> {
    let mut vec = Vec::new();
    for _ in 0..n {
        vec.push(i);
    }
    vec
}
