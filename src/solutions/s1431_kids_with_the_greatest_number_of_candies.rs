/**
 * https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/
 * 
 * There are n kids with candies. You are given an integer array candies, where each
 * candies[i] represents the number of candies the ith kid has, and an integer extraCandies,
 * denoting the number of extra candies that you have.
 *
 * Return a boolean array result of length n, where result[i] is true if, after giving
 * the ith kid all the extraCandies, they will have the greatest number of candies among
 * all the kids, or false otherwise.
 *
 * Note that multiple kids can have the greatest number of candies.
 **/

pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut res: Vec<bool> = vec![];
        let max: i32 = candies.iter().max().unwrap().clone();

        for i in 0..candies.len() {
            res.push(if candies[i] + extra_candies < max {
                false
            } else {
                true
            });
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_kids_with_candies() {
        assert_eq!(
            Solution::kids_with_candies(vec![2, 3, 5, 1, 3], 3),
            vec![true, true, true, false, true]
        );

        assert_eq!(
            Solution::kids_with_candies(vec![4, 2, 1, 1, 2], 1),
            vec![true, false, false, false, false]
        );
    }
}
