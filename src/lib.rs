use std::ops::Mul;

trait Money {
    fn new(amount: u32) -> Self;
    fn amount(&self) -> u32;
}

#[derive(PartialEq, Debug)]
pub struct MoneyBase {
    amount: u32,
}

#[derive(Debug)]
pub struct Dollar {
    money: MoneyBase,
}

#[derive(Debug)]
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

impl PartialEq for Dollar {
    fn eq(&self, other: &Self) -> bool {
        self.amount() == other.amount()
    }
}

impl Mul<u32> for Dollar {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self {
        Self::new(self.amount() * rhs)
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

impl PartialEq for Franc {
    fn eq(&self, other: &Self) -> bool {
        self.amount() == other.amount()
    }
}

impl Mul<u32> for Franc {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self {
        Self::new(self.amount() * rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multiplication() {
        let five = Dollar::new(5);
        assert_eq!(Dollar::new(10), five * 2);
        let five = Dollar::new(5);
        assert_eq!(Dollar::new(15), five * 3);
    }
    #[test]
    fn test_equality() {
        assert_eq!(Dollar::new(5), Dollar::new(5));
        assert!(Dollar::new(5) != Dollar::new(6));
        assert_eq!(Franc::new(5), Franc::new(5));
        assert!(Franc::new(5) != Franc::new(6));
    }
    #[test]
    fn test_franc_multiplication() {
        let five = Franc::new(5);
        assert_eq!(Franc::new(10), five * 2);
        let five = Franc::new(5);
        assert_eq!(Franc::new(15), five * 3);
    }
}
