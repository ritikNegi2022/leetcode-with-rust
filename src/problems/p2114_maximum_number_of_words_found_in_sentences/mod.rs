// 2114. Maximum Number of Words Found in Sentences

struct Solution;

impl Solution {
    pub fn maximum_number_of_words_found_in_sentences(sentences: Vec<String>) -> i32 {
        let mut res: i32 = 0;
        for i in sentences {
            res = res.max(i.split_whitespace().count() as i32)
        }
        res
    }
}

pub fn run() {
    println!("Running problem 2114: Maximum Number of Words Found in Sentences");
    println!(
        "{:?}",
        Solution::maximum_number_of_words_found_in_sentences(vec![
            "alice and bob love leetcode".to_string(),
            "I think so too".to_string(),
            "this is great thanks very much".to_string()
        ])
    );
}
