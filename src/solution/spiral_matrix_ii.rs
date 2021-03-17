pub struct Solution {}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut result = vec![vec![0; n]; n];
        let mut number: i32 = 1;
        let mut y_min: usize = 0;
        let mut y_max: usize = n;
        let mut x_min: usize = 0;
        let mut x_max: usize = n;
        loop {
            for y in y_min..y_max {
                result[x_min][y] = number;
                number += 1;
            }
            x_min += 1;
            if x_min == x_max { break; }
            for x in x_min..x_max {
                result[x][y_max - 1] = number;
                number += 1;
            }
            y_max -= 1;
            if y_min == y_max { break; }
            for y in (y_min..y_max).rev() {
                result[x_max - 1][y] = number;
                number += 1;
            }
            x_max -= 1;
            if x_min == x_max { break; }
            for x in (x_min..x_max).rev() {
                result[x][y_min] = number;
                number += 1;
            }
            y_min += 1;
            if y_min == y_max { break; }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]], Solution::generate_matrix(3));
        assert_eq!(vec![vec![1]], Solution::generate_matrix(1));
    }
}