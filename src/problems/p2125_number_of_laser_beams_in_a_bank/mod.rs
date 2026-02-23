// 2125. Number of Laser Beams in a Bank

struct Solution;

impl Solution {
    pub fn number_of_laser_beams_in_a_bank(bank: Vec<String>) -> i32 {
        let mut res: i32 = 0;
        let mut prev: i32 = 0;
        let mut this: i32 = 0;
        for floor in bank {
            for cell in floor.chars() {
                if cell == '1' {
                    res += prev;
                    this += 1;
                }
            }
            prev = if this == 0 { prev } else { this };
            this = 0;
        }
        res
    }
}

pub fn run() {
    println!("Running problem 2125: Number of Laser Beams in a Bank");
    println!(
        "{:?}",
        Solution::number_of_laser_beams_in_a_bank(vec![
            "011001".to_string(),
            "000000".to_string(),
            "010100".to_string(),
            "001000".to_string()
        ])
    );
}
