struct RLEIterator {
    a: Vec<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RLEIterator {
    fn new(a: Vec<i32>) -> Self {
        return RLEIterator { a };
    }

    fn next(&mut self, n: i32) -> i32 {
        let mut total = n;
        loop {
            if self.a.is_empty() {
                return -1;
            }
            let count = self.a[0];
            if count > total {
                self.a[0] = count - total;
                return self.a[1];
            } else if count == total {
                self.a.remove(0);
                return self.a.remove(0);
            } else if count < total {
                self.a.remove(0);
                self.a.remove(0);
                total = total - count;
            }
        }
    }
}

/**
 * Your RLEIterator object will be instantiated and called as such:
 * let obj = RLEIterator::new(a);
 * let ret_1: i32 = obj.next(n);
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut obj = RLEIterator::new(vec![3, 8, 0, 9, 2, 5]);
        assert_eq!(8, obj.next(2));
        assert_eq!(8, obj.next(1));
        assert_eq!(5, obj.next(1));
        assert_eq!(-1, obj.next(2));
    }

    #[test]
    fn test_2() {
        let mut obj = RLEIterator::new(vec![923381016, 843, 898173122, 924, 540599925, 391, 705283400, 275, 811628709, 850, 895038968, 590, 949764874, 580, 450563107, 660, 996257840, 917, 793325084, 82]);
        assert_eq!(843, obj.next(612783106));
        assert_eq!(924, obj.next(486444202));
        assert_eq!(924, obj.next(630147341));
        assert_eq!(275, obj.next(845077576));
    }
}
