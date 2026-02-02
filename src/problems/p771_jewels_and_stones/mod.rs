// 771. Jewels and Stones

struct Solution;

impl Solution {
    pub fn jewels_and_stones(jewels: String, stones: String) -> i32 {
        let mut total: i32 = 0;
        let mut hash_set: std::collections::HashSet<u8> = std::collections::HashSet::new();
        for b in jewels.bytes() {
            hash_set.insert(b);
        }
        for b in stones.bytes() {
            if hash_set.contains(&b) {
                total += 1;
            }
        }
        total
    }
}

pub fn run() {
    println!("Running problem 771: Jewels and Stones");
    println!(
        "{:?}",
        Solution::jewels_and_stones("aA".to_string(), "aAAbbbb".to_string())
    );
}
