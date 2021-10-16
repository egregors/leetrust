/**
 * https://leetcode.com/problems/shuffle-string/
 *
 * Given a string s and an integer array indices of the same length.
 *
 * The string s will be shuffled such that the character at the ith
 * position moves to indices[i] in the shuffled string.
 *
 **/
#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut res = Vec::with_capacity(s.len());
        unsafe { res.set_len(s.len()) }
        
        for (i, ch) in s.chars().enumerate() {
            res[indices[i] as usize] = ch;
        }

        res.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::restore_string("codeleet".to_string(), vec![4, 5, 6, 7, 0, 2, 1, 3]),
            "leetcode".to_string(),
        );
        assert_eq!(
            Solution::restore_string("abc".to_string(), vec![0, 1, 2]),
            "abc".to_string(),
        );
        assert_eq!(
            Solution::restore_string("aiohn".to_string(), vec![3, 1, 4, 2, 0]),
            "nihao".to_string(),
        );
        assert_eq!(
            Solution::restore_string("aaiougrt".to_string(), vec![4, 0, 2, 6, 7, 3, 1, 5]),
            "arigatou".to_string(),
        );
        assert_eq!(
            Solution::restore_string("art".to_string(), vec![1, 0, 2]),
            "rat".to_string(),
        );
    }
}
