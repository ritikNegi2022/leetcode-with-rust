// 2011. Final Value of Variable After Performing Operations

struct Solution;

impl Solution {
    pub fn final_value_of_variable_after_performing_operations(operations: Vec<String>) -> i32 {
        let mut x: i32 = 0;
        operations
            .iter()
            .for_each(|op| if op.contains("++") { x += 1 } else { x -= 1 });

        x
    }
}

pub fn run() {
    println!("Running problem 2011: Final Value of Variable After Performing Operations");
    println!(
        "{:?}",
        Solution::final_value_of_variable_after_performing_operations(vec![
            "--X".to_string(),
            "X++".to_string(),
            "X++".to_string()
        ])
    );
}
