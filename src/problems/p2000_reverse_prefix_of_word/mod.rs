// 2000. Reverse Prefix of Word

struct Solution;

impl Solution {
    pub fn reverse_prefix_of_word(word: String, ch: char) -> String {
        let mut res: String = String::new();
        for (i, c) in word.chars().enumerate() {
            res = c.to_string() + &res;
            if c == ch {
                res += &word[i + 1..];
                return res;
            }
        }
        word
    }
}

pub fn run() {
    println!("Running problem 2000: Reverse Prefix of Word");
    println!(
        "{:?}",
        Solution::reverse_prefix_of_word("abcdefd".to_string(), 'd')
    );
}
