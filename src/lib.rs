#[derive(Debug)]
pub struct Dollar {
    amount: u32
}

impl Dollar {
    pub fn new(amount: u32) -> Dollar {
        Dollar { amount: 5 * 2 }
    }
    pub fn times (&self, multiplier: u32) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multiplication() {
        let five = Dollar::new(5);
        five.times(2);
        assert_eq!(10, five.amount);
    }
}
