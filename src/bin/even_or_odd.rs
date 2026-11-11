fn even_or_odd(number: i32) -> &'static str {
    match number % 2 {
        0 => "Even",
        _ => "Odd",
    }
}

fn main() {
    even_or_odd(2);
}

#[cfg(test)]
mod tests {
    // This shadows standard assert_eq! with a beautiful colored diff version
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_positive_odd() {
        // If this fails, it prints a clean +/- diff
        assert_eq!(even_or_odd(1), "Odd");
    }
    #[test]
    fn test_zero() {
        assert_eq!(even_or_odd(0), "Even");
    }

    #[test]
    fn test_positive_even() {
        assert_eq!(even_or_odd(2), "Even");
    }

    #[test]
    fn test_negative_even() {
        assert_eq!(even_or_odd(-2), "Even");
    }

    #[test]
    fn test_negative_odd() {
        assert_eq!(even_or_odd(-1), "Odd");
    }
}
