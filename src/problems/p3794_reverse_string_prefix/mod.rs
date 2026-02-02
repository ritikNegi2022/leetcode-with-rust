// 3794. Reverse String Prefix

struct Solution;

impl Solution {
    pub fn reverse_string_prefix(s: String, k: i32) -> String {
        let mut left: usize = 0;
        let mut right: usize = k as usize - 1;
        let mut char_array: Vec<char> = s.chars().collect();
        while left < right {
            char_array.swap(left, right);
            left += 1;
            right -= 1;
        }

        char_array.iter().collect()
    }
}

pub fn run() {
    println!("Running problem 3794: Reverse String Prefix");
    println!(
        "{:?}",
        Solution::reverse_string_prefix("abcd".to_string(), 2)
    );
}
