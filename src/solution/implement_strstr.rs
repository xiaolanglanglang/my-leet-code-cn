pub struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() { return 0; }
        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();
        let mut partial_match_table = vec![0; needle.len()];
        let mut i = 1;
        let mut j = 0;
        while i < needle.len() {
            if needle[i] == needle[j] {
                j += 1;
                partial_match_table[i] = j;
                i += 1;
            } else {
                if j != 0 {
                    j = partial_match_table[j - 1];
                } else {
                    partial_match_table[i] = 0;
                    i += 1;
                }
            }
        }

        let mut i = 0;
        let mut j = 0;
        while i < haystack.len() {
            if haystack[i] == needle[j] {
                i += 1;
                j += 1;
                if j == needle.len() {
                    return (i - needle.len()) as i32;
                }
            } else {
                if j == 0 {
                    i += 1;
                } else {
                    j = partial_match_table[j - 1];
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(-1, Solution::str_str(String::from("aaaaa"), String::from("bba")));
        assert_eq!(2, Solution::str_str(String::from("hello"), String::from("ll")));
        assert_eq!(0, Solution::str_str(String::from(""), String::from("")));
    }
}