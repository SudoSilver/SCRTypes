pub trait BoolExpansion {
    fn flip(&self) -> bool;
}

impl BoolExpansion for bool {
    fn flip(&self) -> bool{
        let flipped = !self;
        return flipped;
    }
}

#[cfg(test)]
mod tests {
    use super::BoolExpansion;

    #[test]
    fn test_flip() {
        assert!(false.flip(), "[TEST]: False did not flip to true");
        assert!(!(true.flip()));
    }
}
