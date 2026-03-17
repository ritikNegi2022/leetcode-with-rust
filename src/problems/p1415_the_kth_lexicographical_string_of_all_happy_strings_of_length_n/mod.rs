// 1415. The k-th Lexicographical String of All Happy Strings of Length n

struct Solution;

impl Solution {
    pub fn the_kth_lexicographical_string_of_all_happy_strings_of_length_n(
        n: i32,
        k: i32,
    ) -> String {
        let mut count: i32 = 1;
        fn f(str: String, n: &i32, k: &i32, count: &mut i32) -> String {
            if str.len() as i32 == *n {
                if count == k {
                    return str;
                } else {
                    *count += 1;
                    return String::from("");
                }
            }
            let last = str.chars().last().unwrap_or(' ');
            let mut res: String;
            if last != 'a' {
                res = f(format!("{str}a"), n, k, count);
                if !res.is_empty() {
                    return res;
                };
            }
            if last != 'b' {
                res = f(format!("{str}b"), n, k, count);
                if !res.is_empty() {
                    return res;
                }
            }
            if last != 'c' {
                res = f(format!("{str}c"), n, k, count);
                if !res.is_empty() {
                    return res;
                }
            }
            String::from("")
        }
        f(String::from(""), &n, &k, &mut count)
    }
}

pub fn run() {
    println!(
        "Running problem 1415: The k-th Lexicographical String of All Happy Strings of Length n"
    );
    println!(
        "{:?}",
        Solution::the_kth_lexicographical_string_of_all_happy_strings_of_length_n(3, 9)
    );
}
