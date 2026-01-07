// 2942. Find Words Containing Character

struct Solution;

impl Solution {
    pub fn find_words_containing_character(words: Vec<String>, x: char) -> Vec<i32> {
        let mut indices: Vec<i32> = Vec::new();
        for (index, word) in words.iter().enumerate() {
            if word.contains(x) {
                indices.push(index as i32);
            }
        }
        indices

        // HACK: one liner answer
        //   words.iter().enumerate().map(|(index, word)| { if word.contains(x) { index as i32 } else { -1_i32 } }).filter(|x| *x > -1).collect()
    }
}

pub fn run() {
    println!("Running problem 2942: Find Words Containing Character");
    println!(
        "{:?}",
        Solution::find_words_containing_character(
            vec!["leet".to_string(), "code".to_string()],
            'e'
        )
    );
}
