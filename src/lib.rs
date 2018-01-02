use std::ops::Mul;
use std::ops::Add;

pub trait Bank {
    fn reduce(source: Sum, to: &'static str) -> Money;
}

#[derive(Debug)]
pub struct Money {
    amount: u32,
    currency: &'static str
}

#[derive(Debug)]
pub struct Sum {
    augend: Money,
    addend: Money
}

impl Money {
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

impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount && self.currency == other.currency
    }
}

impl Mul<u32> for Money {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self {
        Self {
            amount: self.amount * rhs,
            currency: self.currency
        }
    }
}

impl Add for Money {
    type Output = Sum;

    fn add(self, other: Self) -> Sum {
        Sum {
            augend: self,
            addend: other
        }
    }
}

impl Bank for Money {
    fn reduce(source: Sum, to: &'static str) -> Self {
        let amount: u32 = source.augend.amount + source.addend.amount;
        Self {
            amount: amount,
            currency: "USD"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multiplication() {
        let five = Money::dollar(5);
        assert_eq!(Money::dollar(10), five * 2);
        let five = Money::dollar(5);
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
    #[test]
    fn test_simple_addition() {
        let sum: Sum = Money::dollar(5) + Money::dollar(5);
        let reduced = Money::reduce(sum, "USD");
        assert_eq!(Money::dollar(10), reduced);
    }
    #[test]
    fn test_plus_returns_sum() {
        let sum: Sum = Money::dollar(5) + Money::dollar(5);
        assert_eq!(Money::dollar(5), sum.augend);
        assert_eq!(Money::dollar(5), sum.addend);
    }
    #[test]
    fn test_reduce_sum() {
        let sum: Sum = Sum {
            augend: Money::dollar(3),
            addend: Money::dollar(4)
        };
        let result: Money = Money::reduce(sum, "USD");
        assert_eq!(Money::dollar(7), result);
    }
}
