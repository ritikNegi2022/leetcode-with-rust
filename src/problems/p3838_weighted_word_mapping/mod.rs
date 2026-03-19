// 3838. Weighted Word Mapping

struct Solution;

impl Solution {
    pub fn weighted_word_mapping(words: Vec<String>, weights: Vec<i32>) -> String {
        let mut res: String = String::new();
        let mut count: u16 = 0;
        for ch in words.join(",").chars() {
            if ch == ',' {
                count = 25 - (count % 26);
                let char: char = (b'a' + count as u8) as char;
                res.push(char);
                count = 0;
            } else {
                count += weights[(ch as u8 - b'a') as usize] as u16;
            }
        }
        count = 25 - (count % 26);
        let char: char = (b'a' + count as u8) as char;
        res.push(char);
        res
    }
}

pub fn run() {
    println!("Running problem 3838: Weighted Word Mapping");
    println!(
        "{:?}",
        Solution::weighted_word_mapping(
            vec![
                "a".to_string(),
                "ehixqfz".to_string(),
                "rorew".to_string(),
                "cobaelzi".to_string(),
                "ietptx".to_string(),
                "nhp".to_string()
            ],
            vec![
                49, 46, 44, 12, 34, 11, 32, 13, 23, 22, 15, 37, 24, 14, 23, 15, 20, 32, 14, 28, 24,
                35, 50, 41, 22, 27
            ]
        )
    );
}
