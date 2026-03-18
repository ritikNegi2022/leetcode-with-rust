// 1528. Shuffle String

struct Solution;

impl Solution {
    pub fn shuffle_string(s: String, indices: Vec<i32>) -> String {
        let mut res: Vec<char> = s.chars().collect();
        let mut indices_vec: Vec<i32> = Vec::from(indices);
        let n: usize = indices_vec.len();
        for i in 0..n {
            let mut idx: usize = indices_vec[i] as usize;
            if i == idx {
                continue;
            }
            while i != idx as usize {
                res.swap(i, idx);
                indices_vec.swap(i, idx);
                idx = indices_vec[i] as usize;
            }
        }
        res.into_iter().collect::<String>()
    }
}

pub fn run() {
    println!("Running problem 1528: Shuffle String");
    println!(
        "{:?}",
        Solution::shuffle_string("codeleet".to_string(), vec![4, 5, 6, 7, 0, 2, 1, 3])
    );
}
