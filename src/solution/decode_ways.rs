pub struct Solution {}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let char_list: Vec<u32> = s.chars().map(|c| { c.to_digit(10).unwrap() }).collect();
        if char_list[0] == 0 {
            return 0;
        }
        if char_list.len() == 1 {
            return 1;
        }

        let mut dp0 = 1;
        let mut dp1 = Self::v1(char_list[1]) * dp0 + Self::v2(char_list[0] * 10 + char_list[1]);
        let mut dp2;

        for i in 2..char_list.len() {
            dp2 = dp1 * Self::v1(char_list[i]) + dp0 * Self::v2(char_list[i - 1] * 10 + char_list[i]);
            dp0 = dp1;
            dp1 = dp2;
        }
        return dp1 as i32;
    }
    pub fn v1(i: u32) -> u32 {
        return if i >= 1 && i <= 9 { 1 } else { 0 };
    }

    pub fn v2(i: u32) -> u32 {
        return if i >= 10 && i <= 26 { 1 } else { 0 };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::num_decodings(String::from("226")));
        assert_eq!(2, Solution::num_decodings(String::from("12")));
        assert_eq!(0, Solution::num_decodings(String::from("0")));
        assert_eq!(0, Solution::num_decodings(String::from("06")));
        assert_eq!(1836311903, Solution::num_decodings(String::from("111111111111111111111111111111111111111111111")));
    }
}