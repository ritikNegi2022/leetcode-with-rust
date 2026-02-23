// 1678. Goal Parser Interpretation

struct Solution;

impl Solution {
    pub fn goal_parser_interpretation(command: String) -> String {
        let mut res: String = String::new();
        let command_chars: Vec<char> = command.chars().collect();
        let mut i = 0;
        fn parser(command_chars: &Vec<char>, index: usize) -> usize {
            let mut i = index;
            while i < command_chars.len() {
                if command_chars[i] == ')' {
                    return i;
                }
                i += 1;
            }
            i
        }
        while i < command_chars.len() {
            match command_chars[i] {
                'G' => {
                    res += "G";
                }
                _ => {
                    let index = parser(&command_chars, i);
                    if index - i > 1 {
                        res += "al";
                    } else {
                        res += "o";
                    }
                    i = index
                }
            }
            i += 1;
        }
        res
    }
}

pub fn run() {
    println!("Running problem 1678: Goal Parser Interpretation");
    println!(
        "{:?}",
        Solution::goal_parser_interpretation("G()(al)".to_string())
    );
}
