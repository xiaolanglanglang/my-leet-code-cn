pub struct Solution {}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack = vec![];
        let mut n = 0;
        let mut op = '+';
        let last = s.len() - 1;
        for (i, c) in s.chars().enumerate() {
            let is_digit = c.is_ascii_digit();
            if is_digit {
                n = n * 10 + c as i32 - '0' as i32;
            }
            if !is_digit && c != ' ' || i == last {
                match op {
                    '+' => stack.push(n),
                    '-' => stack.push(-1 * n),
                    '*' => {
                        let first_num = stack.pop().unwrap();
                        stack.push(first_num * n);
                    }
                    '/' => {
                        let first_num = stack.pop().unwrap();
                        stack.push(first_num / n);
                    }
                    _ => unreachable!(),
                }
                op = c;
                n = 0;
            }
        };
        stack.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(10, Solution::calculate(String::from("2*3+4")));
        assert_eq!(7, Solution::calculate(String::from("3+2*2")));
        assert_eq!(1, Solution::calculate(String::from(" 3/2 ")));
        assert_eq!(5, Solution::calculate(String::from(" 3+5 / 2 ")));
    }
}

