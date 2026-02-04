// 1684. Count the Number of Consistent Strings

struct Solution;

impl Solution {
    pub fn count_the_number_of_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut res: i32 = 0;
        for word in words.iter() {
            let mut curr_word = word.clone();
            for ch in allowed.chars() {
                curr_word = curr_word.replace(ch, "");
                if curr_word.is_empty() {
                    break;
                }
            }
            if curr_word.is_empty() {
                res += 1;
            }
        }

        res
    }
}

pub fn run() {
    println!("Running problem 1684: Count the Number of Consistent Strings");
    println!(
        "{:?}",
        Solution::count_the_number_of_consistent_strings(
            "cad".to_string(),
            vec![
                "cc".to_string(),
                "acd".to_string(),
                "b".to_string(),
                "ba".to_string(),
                "bac".to_string(),
                "bad".to_string(),
                "ac".to_string(),
                "d".to_string(),
            ]
        )
    );
}
