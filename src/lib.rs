#[derive(Debug)]
pub struct Dollar {
    amount: u32
}

pub struct Franc {
    amount: u32
}

impl Dollar {
    pub fn new (amount: u32) -> Dollar {
        Dollar { amount: amount }
    }
    pub fn times (&self, multiplier: u32) -> Dollar {
        Dollar { amount: self.amount * multiplier }
    }
    pub fn equals (&self, target: Dollar) -> bool {
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
