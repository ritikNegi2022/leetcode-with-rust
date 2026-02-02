// 3541. Find Most Frequent Vowel and Consonant

struct Solution;

impl Solution {
    pub fn find_most_frequent_vowel_and_consonant(s: String) -> i32 {
        let mut freq_array: [i32; 26] = [0; 26];
        let mut max_vouwel: i32 = 0;
        let mut max_constant: i32 = 0;

        for b in s.bytes() {
            let index = b - b'a';
            freq_array[index as usize] += 1;
        }

        for (index, value) in freq_array.iter().enumerate() {
            match index {
                0 | 4 | 8 | 14 | 20 => max_vouwel = max_vouwel.max(*value),
                _ => max_constant = max_constant.max(*value),
            }
        }

        max_vouwel + max_constant
    }
}

pub fn run() {
    println!("Running problem 3541: Find Most Frequent Vowel and Consonant");
    println!(
        "{:?}",
        Solution::find_most_frequent_vowel_and_consonant("successes".to_string())
    );
}
