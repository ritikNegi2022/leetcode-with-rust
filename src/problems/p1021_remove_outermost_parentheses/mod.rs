// 1021. Remove Outermost Parentheses

struct Solution;

impl Solution {
    pub fn remove_outermost_parentheses(s: String) -> String {
        let mut res: String = String::new();
        let mut stack: String = String::new();
        let mut count: i32 = 0;
        for ch in s.chars() {
            stack += &ch.to_string();
            if ch == ')' {
                count -= 1;
            } else {
                count += 1;
            }
            if count == 0 {
                res += &stack[1..stack.len() - 1];
                stack = String::new();
            }
        }
        res
    }
}

pub fn run() {
    println!("Running problem 1021: Remove Outermost Parentheses");
    println!(
        "{:?}",
        Solution::remove_outermost_parentheses("(()())(())".to_string())
    );
}
