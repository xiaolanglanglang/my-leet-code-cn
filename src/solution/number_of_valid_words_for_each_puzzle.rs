pub struct Solution {}

impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let mut results = vec![];
        let mut words_hash = vec![0u16; (2i32.pow(27) - 1) as usize];
        for word in &words {
            let mut pos = 0;
            for w in word.chars() {
                let i = w as i32 - 'a' as i32;
                pos = 1 << i | pos;
            }
            words_hash[pos] += 1;
        }
        for puzzle in &puzzles {
            let f = 1 << (puzzle.chars().nth(0).unwrap() as i32 - 'a' as i32);
            let mut pos = 0;
            for c in puzzle.chars().skip(1) {
                pos |= 1 << (c as i32 - 'a' as i32);
            }
            let mut sub: i32 = pos;
            let mut count = 0;
            loop {
                sub = (sub - 1) & pos;
                count += words_hash[(f | sub) as usize];
                if sub == pos {
                    break;
                }
            }
            results.push(count as i32);
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
