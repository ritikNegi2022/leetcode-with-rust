// 3512. Minimum Operations to Make Array Sum Divisible by K

struct Solution;

impl Solution {
    pub fn solve(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter().sum::<i32>() % k
    }
}

pub fn run() {
    println!("Running problem 3512: Minimum Operations to Make Array Sum Divisible by K");
    println!("{}", Solution::solve(vec![3, 9, 7], 5));
}
