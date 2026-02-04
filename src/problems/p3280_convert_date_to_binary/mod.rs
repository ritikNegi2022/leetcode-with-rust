// 3280. Convert Date to Binary

struct Solution;

impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        let value_array: Vec<&str> = date.split("-").collect();
        let mut res = String::from("");
        for item in value_array {
            let mut int_value: u32 = item.parse().unwrap();
            let mut binary_string = String::from("");
            while int_value > 0 {
                let rem = std::char::from_digit(int_value % 2, 10).unwrap();
                binary_string.insert(0, rem);
                int_value /= 2;
            }
            res = format!("{}-{}", res, binary_string);
        }
        res.remove(0);
        res
    }
}

pub fn run() {
    println!("Running problem 3280: Convert Date to Binary");
    println!(
        "{:?}",
        Solution::convert_date_to_binary("2080-02-29".to_string())
    );
}
