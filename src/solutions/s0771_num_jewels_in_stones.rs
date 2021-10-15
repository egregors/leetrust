/**
 * https://leetcode.com/problems/jewels-and-stones/
 *
 * You're given strings jewels representing the types of stones that are jewels,
 * and stones representing the stones you have. Each character in stones is a type
 * of stone you have. You want to know how many of the stones you have are also
 * jewels.
 *
 * Letters are case sensitive, so "a" is considered a different type of stone
 * from "A".
 *
 **/
#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut res = 0;
        for ch in stones.chars() {
            if jewels.contains(ch) {
                res += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            3,
            Solution::num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string())
        );
        assert_eq!(
            0,
            Solution::num_jewels_in_stones("z".to_string(), "ZZ".to_string())
        );
    }
}
