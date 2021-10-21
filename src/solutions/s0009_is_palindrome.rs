/**
 * https://leetcode.com/problems/palindrome-number/
 *
 * Given an integer x, return true if x is palindrome integer.
 *
 * An integer is a palindrome when it reads the same backward as forward.
 * For example, 121 is palindrome while 123 is not.
 *
**/

struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        x.to_string().chars().rev().collect::<String>() == x.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(true, Solution::is_palindrome(121));
        assert_eq!(false, Solution::is_palindrome(123));
        assert_eq!(true, Solution::is_palindrome(1221));
        assert_eq!(true, Solution::is_palindrome(123321));
        assert_eq!(false, Solution::is_palindrome(-121));
    }
}
