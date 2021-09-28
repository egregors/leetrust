fn main() {
    print!("Sup?");
}

struct Solution {}

impl Solution {
    // https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/
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
