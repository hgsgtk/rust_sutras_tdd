struct Money {
    amount: u32
}

trait MoneyTrait {
    fn new(amount: u32) -> Money;
    fn times(&self, multiplier: u32) -> Money;
    fn equals(&self, target: Money) -> bool;
}

struct Dollar {
    amount: u32
}

pub struct Franc {
    amount: u32
}

impl Money {
    fn times (&self, multiplier: u32) -> Money {
        Money {amount: self.amount * multiplier }
    }
    fn equals(&self, target: Money) -> bool {
        self.amount == target.amount
    }
}

impl MoneyTrait for Dollar {
    fn new (amount: u32) -> Money {
        Money { amount: amount }
    }
    fn times (&self, multiplier: u32) -> Money {
        Money {amount: self.amount * multiplier }
    }
    fn equals (&self, target: Money) -> bool {
        self.amount == target.amount
    }
}

impl Franc {
    pub fn new (amount: u32) -> Franc {
        Franc { amount: amount }
    }
    pub fn times (&self, multiplier: u32) -> Franc {
        Franc {amount: self.amount * multiplier }
    }
    pub fn equals (&self, target: Franc) -> bool {
        self.amount == target.amount
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multiplication() {
        let five = Dollar::new(5);
        assert!(Dollar::new(10).equals(five.times(2)));
        assert!(Dollar::new(15).equals(five.times(3)));
    }
    #[test]
    fn test_equality() {
        assert!(Dollar::new(5).equals(Dollar::new(5)));
        assert!(!Dollar::new(5).equals(Dollar::new(6)));
    }
    #[test]
    fn test_franc_multiplication() {
        let five = Franc::new(5);
        assert!(Franc::new(10).equals(five.times(2)));
        assert!(Franc::new(15).equals(five.times(3)));
    }
}
