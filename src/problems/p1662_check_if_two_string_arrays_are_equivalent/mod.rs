// 1662. Check If Two String Arrays are Equivalent

struct Solution;

impl Solution {
    pub fn check_if_two_string_arrays_are_equivalent(
        word1: Vec<String>,
        word2: Vec<String>,
    ) -> bool {
        word1.join("") == word2.join("")
    }
}

pub fn run() {
    println!("Running problem 1662: Check If Two String Arrays are Equivalent");
    println!(
        "{:?}",
        Solution::check_if_two_string_arrays_are_equivalent(
            vec!["ab".to_string(), "c".to_string()],
            vec!["a".to_string(), "bc".to_string()]
        )
    );
}
