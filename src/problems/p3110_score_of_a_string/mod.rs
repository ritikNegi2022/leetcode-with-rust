// 3110. Score of a String
struct Solution;

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let mut score: i32 = 0;
        let bytes: Vec<u8> = s.bytes().collect::<Vec<u8>>();
        let n: usize = s.len();
        let mut i: usize = 1;
        while i < n {
            if bytes[i - 1] > bytes[i] {
                score += bytes[i - 1] as i32 - bytes[i] as i32
            } else {
                score += bytes[i] as i32 - bytes[i - 1] as i32
            }
            i += 1;
        }
        score
    }
}

pub fn run() {
    println!("Running problem 3110: Score of a String");
    println!("{:?}", Solution::score_of_string("hello".to_string()));
}
