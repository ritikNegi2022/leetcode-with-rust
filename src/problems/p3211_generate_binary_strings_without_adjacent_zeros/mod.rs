// 3211. Generate Binary Strings Without Adjacent Zeros

struct Solution;

impl Solution {
    pub fn generate_binary_strings_without_adjacent_zeros(n: i32) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        fn recursive(s: &mut String, res: &mut Vec<String>, n: &usize) {
            if s.len() == *n {
                res.push(s.to_string());
                return;
            }
            s.push('1');
            recursive(s, res, n);
            s.pop();
            if s.is_empty() || s.ends_with('1') {
                s.push('0');
                recursive(s, res, n);
                s.pop();
            }
        }
        recursive(&mut String::new(), &mut res, &(n as usize));
        res
    }
}

pub fn run() {
    println!("Running problem 3211: Generate Binary Strings Without Adjacent Zeros");
    println!(
        "{:?}",
        Solution::generate_binary_strings_without_adjacent_zeros(3)
    );
}
