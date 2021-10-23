/*
 * https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/
 *
 * Suppose an array of length n sorted in ascending order is rotated
 * between 1 and n times. For example, the array nums = [0,1,4,4,5,6,7]
 * might become:
 *,
 * [4,5,6,7,0,1,4] if it was rotated 4 times.
 * [0,1,4,4,5,6,7] if it was rotated 7 times.
 *
 * Notice that rotating an array [a[0], a[1], a[2], ..., a[n-1]]
 * 1 time results in the array [a[n-1], a[0], a[1], a[2], ..., a[n-2]].
 *
 * Given the sorted rotated array nums that may contain duplicates,
 * return the minimum element of this array.
 *
**/
#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        let mut m;

        while l < r {
            m = l + (r - l) / 2;
            if nums[m] > nums[r] {
                l = m + 1;
            } else if nums[m] < nums[r] {
                r = m;
            } else {
                r -= 1;
            }
        }
        nums[l]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::find_min(vec![2, 2, 2, 0, 1]), 0);
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 4]), 0);
        assert_eq!(Solution::find_min(vec![1, 3, 5]), 1);
        assert_eq!(Solution::find_min(vec![10, 1, 10, 10, 10]), 1);
    }
}
