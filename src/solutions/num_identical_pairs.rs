/**
 * https://leetcode.com/problems/number-of-good-pairs/
 *
 * Given an array of integers nums, return the number of good pairs.
 *
 * A pair (i, j) is called good if nums[i] == nums[j] and i < j.
*/

pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut good_pairs = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] == nums[j] && i < j {
                    good_pairs += 1;
                }
            }
        }
        return good_pairs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_identical_pairs() {
        assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
        assert_eq!(Solution::num_identical_pairs(vec![1, 1, 1, 1]), 6);
        assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3]), 0);
    }
}
