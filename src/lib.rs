pub struct Money {
    amount: u32,
    currency: &'static str
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
        Money {
            amount: &self.amount * multiplier,
            currency: &self.currency
        }
    }
    pub fn equals (&self, target: Money) -> bool {
        self.amount == target.amount && self.currency == target.currency
    }
    pub fn dollar (amount: u32) -> Money {
        Money { 
            amount: amount,
            currency: "USD"
        }
    }
    pub fn franc (amount: u32) -> Money {
        Money { 
            amount: amount,
            currency: "CHF"
        }
    }
    pub fn currency (&self) -> &'static str {
        self.currency
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
        assert!(Money::franc(5).equals(Money::franc(5)));
        assert!(!Money::franc(5).equals(Money::franc(6)));
        assert!(!Money::franc(5).equals(Money::dollar(5)));
    }
    #[test]
    fn test_franc_multiplication() {
        let five = Money::franc(5);
        assert!(Money::franc(10).equals(five.times(2)));
        assert!(Money::franc(15).equals(five.times(3)));
    }
    #[test]
    fn test_currency() {
        assert_eq!("USD", Money::dollar(1).currency());
        assert_eq!("CHF", Money::franc(1).currency());
    }
}
