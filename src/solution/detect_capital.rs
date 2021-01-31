pub struct Solution {}

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut is_first = true;
        let mut upper_word_count = 0;
        let mut has_lower_word = false;
        for c in word.chars() {
            if c.is_uppercase() {
                if !is_first && has_lower_word {
                    return false;
                }
                upper_word_count += 1;
            } else {
                has_lower_word = true;
                if upper_word_count > 1 {
                    return false;
                }
            }
            if is_first {
                is_first = false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::detect_capital_use(String::from("USA")));
        assert_eq!(false, Solution::detect_capital_use(String::from("FlaG")));
        assert_eq!(false, Solution::detect_capital_use(String::from("FFFFFFFFFFFFFFFFFFFFf")));
    }
}