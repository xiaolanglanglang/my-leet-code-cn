pub struct Solution {}

impl Solution {
    pub fn one_edit_away(first: String, second: String) -> bool {
        let shorter;
        let longer;
        if first.len() <= second.len() {
            shorter = first.as_bytes();
            longer = second.as_bytes();
        } else {
            shorter = second.as_bytes();
            longer = first.as_bytes();
        }
        return match longer.len() - shorter.len() {
            0 => {
                let mut one_diff = false;
                for i in 0..shorter.len() {
                    if shorter[i] != longer[i] {
                        if one_diff {
                            return false;
                        } else {
                            one_diff = true;
                        }
                    }
                }
                true
            }
            1 => {
                let mut diff = 0usize;
                for i in 0..shorter.len() {
                    if shorter[i] != longer[i + diff] {
                        if diff == 0 {
                            if shorter[i] == longer[i + 1] {
                                diff = 1;
                            } else {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }
                }
                true
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::one_edit_away(String::from("pale"), String::from("ple")));
    }

    #[test]
    fn test_2() {
        assert_eq!(false, Solution::one_edit_away(String::from("pales"), String::from("pal")));
    }

    #[test]
    fn test_3() {
        assert_eq!(true, Solution::one_edit_away(String::from(""), String::from("a")));
    }

    #[test]
    fn test_4() {
        assert_eq!(true, Solution::one_edit_away(String::from("a"), String::from("ab")));
    }

    #[test]
    fn test_5() {
        assert_eq!(true, Solution::one_edit_away(String::from("ab"), String::from("a")));
    }

    #[test]
    fn test_6() {
        assert_eq!(false, Solution::one_edit_away(String::from("teacher"), String::from("bleacher")));
    }

    #[test]
    fn test_7() {
        assert_eq!(true, Solution::one_edit_away(String::from("islander"), String::from("slander")));
    }
}
