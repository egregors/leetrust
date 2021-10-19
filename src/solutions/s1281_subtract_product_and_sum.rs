/**
 * https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/
 *
 * Given an integer number n, return the difference between the product of its digits and
 * the sum of its digits.
 *
**/

struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let n = n.to_string();
        let xs = n.chars().map(|x| x.to_digit(10).unwrap());
        (xs.clone().product::<u32>() - xs.sum::<u32>()) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(15, Solution::subtract_product_and_sum(234));
        assert_eq!(21, Solution::subtract_product_and_sum(4421));
    }
}
