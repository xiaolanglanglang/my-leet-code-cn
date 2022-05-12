pub struct Solution {}

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let row = strs.len();
        let col = strs[0].len();
        let mut vec = vec![];
        for s in &strs {
            vec.push(s.as_bytes());
        }
        let mut ans = 0;
        for x in 0..col {
            for y in 1..row {
                if vec[y][x] < vec[y - 1][x] {
                    ans += 1;
                    break;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::min_deletion_size(vec!["cba".to_string(), "daf".to_string(), "ghi".to_string()]));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::min_deletion_size(vec!["a".to_string(), "b".to_string()]));
    }

    #[test]
    fn test_3() {
        assert_eq!(3, Solution::min_deletion_size(vec!["zyx".to_string(), "wvu".to_string(), "tsr".to_string()]));
    }

    #[test]
    fn test_4() {
        assert_eq!(2, Solution::min_deletion_size(vec!["rrjk".to_string(), "furt".to_string(), "guzm".to_string()]));
    }
}