// 1221. Split a String in Balanced Strings

struct Solution;

impl Solution {
    pub fn split_a_string_in_balanced_strings(s: String) -> i32 {
        let mut count: i32 = 0;
        let mut res: i32 = 0;
        for ch in s.chars() {
            if ch == 'L' {
                count -= 1;
            } else {
                count += 1;
            }
            if count == 0 {
                res += 1;
            }
        }
        res
    }
}

pub fn run() {
    println!("Running problem 1221: Split a String in Balanced Strings");
    println!(
        "{:?}",
        Solution::split_a_string_in_balanced_strings("RLRRLLRLRL".to_string())
    );
}
