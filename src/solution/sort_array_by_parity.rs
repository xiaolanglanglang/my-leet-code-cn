pub struct Solution {}

impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut res = nums;
        let mut i = 0;
        for j in 0..res.len() {
            if res[j] & 1 == 0 {
                res.swap(i, j);
                i += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![2, 4, 3, 1], Solution::sort_array_by_parity(vec![3, 1, 2, 4]));
    }
}