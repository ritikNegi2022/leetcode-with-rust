// 2325. Decode the Message

struct Solution;

impl Solution {
    pub fn decode_the_message(key: String, message: String) -> String {
        let mut res: String = String::new();
        let chars_arr: Vec<char> = key.replace(" ", "").chars().collect();
        let og_chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        let mut char_map: std::collections::HashMap<char, char> = std::collections::HashMap::new();
        let mut index: usize = 0;
        for ch in chars_arr {
            if !char_map.contains_key(&ch) {
                char_map.insert(ch, og_chars[index]);
                index += 1;
            }
            if char_map.len() >= 26 {
                break;
            }
        }
        for i in message.chars() {
            if let Some(v) = char_map.get(&i) {
                res += &v.to_string();
            } else {
                res += &i.to_string();
            }
        }
        res
    }
}

pub fn run() {
    println!("Running problem 2325: Decode the Message");
    println!(
        "{:?}",
        Solution::decode_the_message(
            "the quick brown fox jumps over the lazy dog".to_string(),
            "vkbs bs t suepuv".to_string()
        )
    );
}
