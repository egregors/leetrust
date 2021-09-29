/**
 * https://leetcode.com/problems/design-parking-system/
 *
 * Design a parking system for a parking lot. The parking lot has three kinds of parking spaces:
 * big, medium, and small, with a fixed number of slots for each size.
 *
 * Implement the ParkingSystem class:
 *
 *    ParkingSystem(int big, int medium, int small) Initializes object of the ParkingSystem class.
 *    The number of slots for each parking space are given as part of the constructor.
 *    bool addCar(int carType) Checks whether there is a parking space of carType for the car that wants
 *    to get into the parking lot. carType can be of three kinds: big, medium, or small, which are
 *    represented by 1, 2, and 3 respectively. A car can only park in a parking space of its carType.
 *    If there is no space available, return false, else park the car in that size space and return true.
*/
#[allow(dead_code)]
struct ParkingSystem {
    big: i32,
    medium: i32,
    small: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self { big, medium, small }
    }
    fn add_car(&mut self, car_type: i32) -> bool {
        let res: bool;
        match car_type {
            1 => {
                self.big -= 1;
                res = if self.big >= 0 { true } else { false };
            }
            2 => {
                self.medium -= 1;
                res = if self.medium >= 0 { true } else { false };
            }
            3 => {
                self.small -= 1;
                res = if self.small >= 0 { true } else { false };
            }
            _ => res = false,
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let mut obj = ParkingSystem::new(1, 1, 0);
        let cars = vec![1, 2, 3, 1];
        let res = cars
            .into_iter()
            .map(|car_type| obj.add_car(car_type))
            .collect::<Vec<_>>();
        assert_eq!(res, vec![true, true, false, false]);
    }
}
