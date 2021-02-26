pub struct Solution {}

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut new_matrix = vec![vec![0; matrix.len()]; matrix[0].len()];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                new_matrix[j][i] = matrix[i][j];
            }
        }
        new_matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]], Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]));
    }
}