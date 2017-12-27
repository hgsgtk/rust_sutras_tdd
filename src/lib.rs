#[derive(PartialEq, Debug)]
pub struct Money {
    amount: u32
}

trait MoneyTrait {
    fn new(amount: u32) -> Money;
}

pub struct Dollar {
}

pub struct Franc {
}

impl Money {
    pub fn times (&self, multiplier: u32) -> Money {
        Money {amount: self.amount * multiplier }
    }
    pub fn equals(&self, target: Money) -> bool {
        self.amount == target.amount
    }
}

impl MoneyTrait for Dollar {
    fn new (amount: u32) -> Money {
        Money { amount: amount }
    }
}

impl Franc {
    pub fn new (amount: u32) -> Money {
        Money { amount: amount }
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
