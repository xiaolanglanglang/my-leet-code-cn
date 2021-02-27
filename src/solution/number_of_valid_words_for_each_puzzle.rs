pub struct Solution {}

impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let mut results = vec![];
        let mut words_hash = vec![];
        for word in &words {
            let mut word_hash = 0;
            for w in word.chars() {
                let pos = w as i32 - 'a' as i32;
                word_hash = 1 << pos | word_hash;
            }
            words_hash.push(word_hash);
        }
        for puzzle in &puzzles {
            let mut hash = 0;
            let mut first = -1;
            for char in puzzle.chars() {
                let pos = char as i32 - 'a' as i32;
                if first == -1 {
                    first = 1 << pos;
                }
                hash = 1 << pos | hash;
            }
            let mut result = 0;
            for &word_hash in &words_hash {
                if (word_hash & hash) != word_hash {
                    continue;
                }
                if (first & word_hash) == first {
                    result += 1;
                }
            }
            results.push(result);
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![1, 1, 3, 2, 4, 0], Solution::find_num_of_valid_words(vec!["aaaa".to_string(), "asas".to_string(), "able".to_string(), "ability".to_string(), "actt".to_string(), "actor".to_string(), "access".to_string()], vec!["aboveyz".to_string(), "abrodyz".to_string(), "abslute".to_string(), "absoryz".to_string(), "actresz".to_string(), "gaswxyz".to_string()]));
    }
}
