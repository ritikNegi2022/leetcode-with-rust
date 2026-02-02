// 1689. Partitioning Into Minimum Number Of Deci-Binary Numbers

struct Solution;

impl Solution {
    // NOTE: this is solution is mine own.
    pub fn partitioning_into_minimum_number_of_decibinary_numbers(n: String) -> i32 {
        let mut max: i32 = 0;
        for i in n.chars() {
            let value: i32 = i.to_string().parse().unwrap();
            max = max.max(value);
        }
        max
    }
    // HACK: this is most efficient method.
    // pub fn partitioning_into_minimum_number_of_decibinary_numbers(n: String) -> i32 {
    //     let mut max = 0;
    //     for b in n.bytes() {
    //         max = max.max(b - b'0');
    //     }
    //     max as i32
    // }
}

pub fn run() {
    println!("Running problem 1689: Partitioning Into Minimum Number Of Deci-Binary Numbers");
    println!(
        "{:?}",
        Solution::partitioning_into_minimum_number_of_decibinary_numbers(
            "27346209830709182346".to_string()
        )
    );
}
