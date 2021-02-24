pub struct Solution {}

impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();
        let mut compare_list = vec![0 as i32; s.len()];
        for i in 0..s.len() {
            if s_bytes[i] >= t_bytes[i] {
                compare_list[i] = (s_bytes[i] - t_bytes[i]) as i32;
            } else {
                compare_list[i] = (t_bytes[i] - s_bytes[i]) as i32;
            }
        }
        let mut current_cost = max_cost;
        let mut left_pointer = 0;
        let mut right_pointer = 0;
        while right_pointer < compare_list.len() {
            current_cost -= compare_list[right_pointer];
            if current_cost < 0 {
                current_cost += compare_list[left_pointer];
                left_pointer += 1;
            }
            right_pointer += 1;
        }
        return (right_pointer - left_pointer) as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::equal_substring(String::from("vjlqwkzamvyv"), String::from("suusjpqkhlkz"), 7));
    }

    #[test]
    fn test_2() {
        assert_eq!(3, Solution::equal_substring(String::from("abcd"), String::from("bcdf"), 3));
    }

    #[test]
    fn test_3() {
        assert_eq!(1, Solution::equal_substring(String::from("abcd"), String::from("cdef"), 3));
    }

    #[test]
    fn test_4() {
        assert_eq!(1, Solution::equal_substring(String::from("abcd"), String::from("acde"), 0));
    }

    #[test]
    fn test_5() {
        assert_eq!(4, Solution::equal_substring(String::from("krpgjbjjznpzdfy"), String::from("nxargkbydxmsgby"), 14));
    }

    #[test]
    fn test_6() {
        assert_eq!(4, Solution::equal_substring(String::from("abcd"), String::from("abcd"), 0));
    }
}