mod main_981608139;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_numbers() {
        assert_eq!(main_981608139::is_prime(2), true);
        assert_eq!(main_981608139::is_prime(3), true);
        assert_eq!(main_981608139::is_prime(5), true);
        assert_eq!(main_981608139::is_prime(7), true);
        assert_eq!(main_981608139::is_prime(11), true);
        assert_eq!(main_981608139::is_prime(13), true);
        assert_eq!(main_981608139::is_prime(17), true);
    }

    #[test]
    fn test_non_prime_numbers() {
        assert_eq!(main_981608139::is_prime(0), false);
        assert_eq!(main_981608139::is_prime(1), false);
        assert_eq!(main_981608139::is_prime(4), false);
        assert_eq!(main_981608139::is_prime(6), false);
        assert_eq!(main_981608139::is_prime(8), false);
        assert_eq!(main_981608139::is_prime(9), false);
        assert_eq!(main_981608139::is_prime(15), false);
    }
}