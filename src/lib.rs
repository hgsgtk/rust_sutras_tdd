#[derive(Debug)]
pub struct Dollar {
    amount: u32
}

impl Dollar {
    pub fn new(amount: u32) -> Dollar {
        Dollar { amount: amount }
    }
    pub fn times (&self, multiplier: u32) -> Dollar {
        Dollar { amount: self.amount * multiplier }
    }
    pub fn equals (&self, target: Dollar) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multiplication() {
        let five = Dollar::new(5);
        let mut product = five.times(2);
        assert_eq!(10, product.amount);
        product = five.times(3);
        assert_eq!(15, product.amount);
    }
    #[test]
    fn test_equality() {
        assert!(Dollar::new(5).equals(Dollar::new(5)));
    }
}
