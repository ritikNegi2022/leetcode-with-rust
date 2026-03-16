// 2375. Construct Smallest Number From DI String

struct Solution;

impl Solution {
    pub fn construct_smallest_number_from_di_string(pattern: String) -> String {
        let mut res: String = String::new();
        let mut stack: Vec<usize> = Vec::new();
        for (i, ch) in pattern.chars().enumerate() {
            if ch == 'D' {
                stack.push(i + 1);
            } else {
                res += &(i + 1).to_string();
                while !stack.is_empty() {
                    let n = stack.pop().unwrap();
                    res += &n.to_string();
                }
            }
        }
        res += &(pattern.len() + 1).to_string();
        while !stack.is_empty() {
            let n = stack.pop().unwrap();
            res += &n.to_string();
        }
        res
    }
}

pub fn run() {
    println!("Running problem 2375: Construct Smallest Number From DI String");
    println!(
        "{:?}",
        Solution::construct_smallest_number_from_di_string("IIIDIDDD".to_string())
    );
}
