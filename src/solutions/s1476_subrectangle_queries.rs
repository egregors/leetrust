/**
 * https://leetcode.com/problems/subrectangle-queries/
 *
 * Implement the class SubrectangleQueries which receives a rows x cols
 * rectangle as a matrix of integers in the constructor and supports two methods:
 *
 *  1. updateSubrectangle(int row1, int col1, int row2, int col2, int newValue)
 *  Updates all values with newValue in the subrectangle whose upper left coordinate
 *  is (row1,col1) and bottom right coordinate is (row2,col2).
 *
 *   2. getValue(int row, int col)
 *  Returns the current value of the coordinate (row,col) from the rectangle.
 **/

struct SubrectangleQueries {
    r: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl SubrectangleQueries {
    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        SubrectangleQueries { r: rectangle }
    }

    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        for i in row1..=row2 {
            for j in col1..=col2 {
                self.r[i as usize][j as usize] = new_value;
            }
        }
    }

    fn get_value(&self, row: i32, col: i32) -> i32 {
        self.r[row as usize][col as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_all() {
        let mut obj = SubrectangleQueries::new(vec![
            vec![1, 2, 1],
            vec![4, 3, 4],
            vec![3, 2, 1],
            vec![1, 1, 1],
        ]);
        assert_eq!(obj.get_value(0, 2), 1);
        obj.update_subrectangle(0, 0, 3, 2, 5);
        assert_eq!(obj.get_value(0, 2), 5);
        assert_eq!(obj.get_value(3, 1), 5);
        obj.update_subrectangle(3, 0, 3, 2, 10);
        assert_eq!(obj.get_value(3, 1), 10);
        assert_eq!(obj.get_value(0, 2), 5);
    }
}
