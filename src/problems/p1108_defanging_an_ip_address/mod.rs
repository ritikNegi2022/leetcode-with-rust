// 1108. Defanging an IP Address

struct Solution;

impl Solution {
    pub fn defanging_an_ip_address(ip: String) -> String {
        ip.as_str().replacen(".", "[.]", 3)
    }
}

pub fn run() {
    println!("Running problem 1108: Defanging an IP Address");
    println!(
        "{:?}",
        Solution::defanging_an_ip_address("0.0.0.0".to_string())
    );
}
