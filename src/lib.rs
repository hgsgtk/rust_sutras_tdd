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
    pub fn dollar(amount: u32) -> Money {
        Money { amount: amount }
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
        let five = Money::dollar(5);
        assert!(Money::dollar(10).equals(five.times(2)));
        assert!(Money::dollar(15).equals(five.times(3)));
    }
    #[test]
    fn test_equality() {
        assert!(Money::dollar(5).equals(Money::dollar(5)));
        assert!(!Money::dollar(5).equals(Money::dollar(6)));
        assert!(Franc::new(5).equals(Franc::new(5)));
        assert!(!Franc::new(5).equals(Franc::new(6)));
        //TODO: assert!(!Franc::new(5).equals(Money::dollar(5)));
    }
    #[test]
    fn test_franc_multiplication() {
        let five = Franc::new(5);
        assert!(Franc::new(10).equals(five.times(2)));
        assert!(Franc::new(15).equals(five.times(3)));
    }
}
