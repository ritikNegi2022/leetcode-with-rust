// 3760. Maximum Substrings With Distinct Start

struct Solution;

impl Solution {
    pub fn max_distinct(s: String) -> i32 {
        let mut count: i32 = 0;
        let mut set: [bool; 26] = [false; 26];
        let chars = s.bytes();
        for ch in chars {
            let index: usize = (ch - b'a') as usize;
            if !set[index] {
                set[index] = true;
                count += 1;
            }
        }
        count
    }
}

pub fn run() {
    println!("Running problem 3760: Maximum Substrings With Distinct Start");
    println!("{:?}", Solution::max_distinct("abab".to_string()));
}
