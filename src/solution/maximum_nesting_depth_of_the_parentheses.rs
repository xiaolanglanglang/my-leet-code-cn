pub struct Solution {}

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut max_depth = 0;
        let mut depth = 0;
        for b in s.chars() {
            if b == '(' {
                depth += 1;
                if depth > max_depth {
                    max_depth = depth;
                }
            }
            if b == ')' {
                depth -= 1;
            }
        }
        return max_depth;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::max_depth(String::from("(1+(2*3)+((8)/4))+1")));
    }
}