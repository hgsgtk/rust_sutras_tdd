use std::ops::Mul;

trait Money {
    fn new(amount: u32) -> Self;
    fn amount(&self) -> u32;
}

#[derive(PartialEq, Debug)]
pub struct MoneyBase {
    amount: u32,
}

pub struct Dollar {
    money: MoneyBase,
}

pub struct Franc {
    money: MoneyBase,
}

impl Money for Dollar {
    fn new(amount: u32) -> Self {
        Self { money: MoneyBase { amount } }
    }
    fn amount(&self) -> u32 {
        self.money.amount
    }
}

impl Mul for Dollar {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(self.amount() * other.amount())
    }
}

impl Money for Franc {
    fn new(amount: u32) -> Self {
        Self { money: MoneyBase { amount } }
    }
    fn amount(&self) -> u32 {
        self.money.amount
    }
}

impl Mul for Franc {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(self.amount() * other.amount())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multiplication() {
        let five = Dollar::new(5);
        assert_eq!(Dollar::new(10), five * 2);
        assert_eq!(Dollar::new(15), five * 3);
    }
    #[test]
    fn test_equality() {
        assert_eq!(Dollar::new(5), Dollar::new(5));
        assert!(Dollar::new(5) != Dollar::new(6));
        assert_eq!(Franc::new(5), Franc::new(5));
        assert_eq!(Franc::new(5) != Franc::new(6));
    }
    #[test]
    fn test_franc_multiplication() {
        let five = Franc::new(5);
        assert_eq!(Franc::new(10), five * 2);
        assert_eq!(Franc::new(15), five * 3);
    }
}
