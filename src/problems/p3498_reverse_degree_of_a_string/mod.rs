// 3498. Reverse Degree of a String

struct Solution;

impl Solution {
    pub fn reverse_degree_of_a_string(s: String) -> i32 {
        let mut res: i32 = 0;
        for (i, b) in s.bytes().enumerate() {
            let o_index: i32 = 26 - (b - b'a') as i32;
            res += (i as i32 + 1) * o_index;
        }
        res
    }
}

pub fn run() {
    println!("Running problem 3498: Reverse Degree of a String");
    println!(
        "{:?}",
        Solution::reverse_degree_of_a_string("abc".to_string())
    );
}
