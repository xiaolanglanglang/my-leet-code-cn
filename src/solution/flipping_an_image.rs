pub struct Solution {}

impl Solution {
    pub fn flip_and_invert_image(mut a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for line in &mut a {
            let mut left_pointer = 0;
            let mut right_pointer = line.len() - 1;
            let mut temp;
            while left_pointer <= right_pointer {
                if left_pointer == right_pointer {
                    line[left_pointer] = 1 - line[left_pointer];
                } else {
                    temp = line[left_pointer];
                    line[left_pointer] = 1 - line[right_pointer];
                    line[right_pointer] = 1 - temp;
                }
                left_pointer += 1;
                if right_pointer != 0 {
                    right_pointer -= 1;
                }
            }
        }
        return a;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]], Solution::flip_and_invert_image(vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]]));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![vec![1, 1, 0, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 1], vec![1, 0, 1, 0]], Solution::flip_and_invert_image(vec![vec![1, 1, 0, 0], vec![1, 0, 0, 1], vec![0, 1, 1, 1], vec![1, 0, 1, 0]]));
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![vec![0]], Solution::flip_and_invert_image(vec![vec![1]]));
    }
}