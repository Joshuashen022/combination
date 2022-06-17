use factorial::Factorial;
use std::ops::{Div, Mul, Sub};

pub struct Combination<T>
where
    T: Factorial + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Clone + PartialOrd,
{
    n: T,
    r: T,
}

impl<T> Combination<T>
where
    T: Factorial + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Clone + PartialOrd,
{
    pub fn new(n: T, r: T) -> Combination<T> {
        Combination { n, r }
    }

    /// Before getting the result,
    /// check if 'n >= r' or overflow happened during calculating factorial.
    pub fn try_result(&self) -> Result<T, &str> {
        if self.n < self.r {
            return Err("N should not be less than R");
        }

        let top = if let Some(top) = self.n.checked_factorial() {
            top
        } else {
            return Err("Overflowed at top factorial");
        };

        let bot_left = if let Some(bot_left) = self.r.checked_factorial() {
            bot_left
        } else {
            return Err("Overflowed at bottom left factorial");
        };

        let bot_right =
            if let Some(bot_right) = (self.n.clone() - self.r.clone()).checked_factorial() {
                bot_right
            } else {
                return Err("Overflowed at bottom right factorial");
            };

        Ok(top / (bot_left * bot_right))
    }

    /// Get the result without check anything.
    /// It's fast but could panic.
    pub fn get_result_uncheck(&self) -> T {
        let top = self.n.factorial();
        let bot_left = self.r.factorial();
        let bot_right = (self.n.clone() - self.r.clone()).factorial();
        top / (bot_left * bot_right)
    }
}

#[test]
fn test_combination_uncheck() {
    let s = Combination::new(10u32, 2);
    let result = s.get_result_uncheck();
    println!("{}", result);
}

#[test]
fn test_combination_with_check() {
    let s = Combination::new(30u128, 10);
    let result = s.try_result();
    println!("{:?}", result);
}

#[test]
fn big_num_test() {
    use num::bigint::BigUint;
    let c = BigUint::parse_bytes(b"50", 10).unwrap();
    let r = BigUint::parse_bytes(b"30", 10).unwrap();

    let comb = Combination::new(c, r);

    let res = comb.try_result();
    assert_eq!(
        res,
        Ok(BigUint::parse_bytes(b"47129212243960", 10).unwrap())
    );
}
