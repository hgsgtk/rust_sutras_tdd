trait Money {
    fn new(amount: u32) -> Self;
    fn amount(&self) -> u32;
    fn times(&self, multiplier: u32) -> Self;
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
    fn times(&self, multiplier: u32) -> Self {
        Self::new(self.amount() * multiplier)
    }
}

impl Money for Franc {
    fn new(amount: u32) -> Self {
        Self { money: MoneyBase { amount } }
    }
    fn amount(&self) -> u32 {
        self.money.amount
    }
    fn times(&self, multiplier: u32) -> Self {
        Self::new(self.amount() * multiplier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multiplication() {
        let five = Dollar::new(5);
        assert_eq!(Dollar::new(10), five.times(2));
        assert_eq!(Dollar::new(15), five.times(3));
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
        assert_eq!(Franc::new(10), (five.times(2)));
        assert_eq!(Franc::new(15), (five.times(3)));
    }
}
