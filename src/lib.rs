use std::ops::Mul;

#[derive(PartialEq, Debug)]
pub struct Money {
    amount: u32,
    currency: &'static str
}

impl Money {
    fn new(amount: u32, currency: &'static str) -> Self {
        Self { 
            amount: amount,
            currency: currency
        }
    }
    fn dollar (amount: u32) -> Money {
        Self { 
            amount: amount,
            currency: "USD"
        }
    }
    fn franc (amount: u32) -> Money {
        Self { 
            amount: amount,
            currency: "CHF"
        }
    }
    fn currency (&self) -> &'static str {
        self.currency
    }
}

impl Mul for Money {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(self.amount * other.amount, self.currency())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multiplication() {
        let five = Money::dollar(5);
        assert_eq!(Money::dollar(10), five * 2);
        assert_eq!(Money::dollar(15), five * 3);
    }
    #[test]
    fn test_equality() {
        assert_eq!(Money::dollar(5), Money::dollar(5));
        assert!(Money::dollar(5) != Money::dollar(6));
        assert!(Money::franc(5) != Money::dollar(5));
    }
    #[test]
    fn test_currency() {
        assert_eq!("USD", Money::dollar(1).currency());
        assert_eq!("CHF", Money::franc(1).currency());
    }
}
