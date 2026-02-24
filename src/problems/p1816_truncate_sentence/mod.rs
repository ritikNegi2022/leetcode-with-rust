// 1816. Truncate Sentence

struct Solution;

impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let mut res: String = String::new();
        for i in s.split_whitespace() {
            res += i;
            if res.split_whitespace().count() as i32 == k {
                break;
            }
            res += " ";
        }

        res
    }
}

pub fn run() {
    println!("Running problem 1816: Truncate Sentence");
    println!(
        "{:?}",
        Solution::truncate_sentence("Hello how are you Contestant".to_string(), 4)
    );
}
