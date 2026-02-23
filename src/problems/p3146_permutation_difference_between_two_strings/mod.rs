// 3146. Permutation Difference between Two Strings

struct Solution;

impl Solution {
    pub fn permutation_difference_between_two_strings(s: String, t: String) -> i32 {
        let mut res: i32 = 0;
        let mut map: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
        for (i, ch) in s.chars().enumerate() {
            map.insert(ch, i);
        }
        for (i, ch) in t.chars().enumerate() {
            if let Some(index) = map.get(&ch) {
                res += (*index as i32 - i as i32).abs();
            }
        }
        res
    }
}

pub fn run() {
    println!("Running problem 3146: Permutation Difference between Two Strings");
    println!(
        "{:?}",
        Solution::permutation_difference_between_two_strings("abc".to_string(), "bac".to_string())
    );
}
