pub fn absolute_difference(n1: &i32, n2: &i32) -> i32 {
    if n1 > n2 {
        n1 - n2
    } else {
        n2 - n1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_calculate_absolute_difference() {
        let result = absolute_difference(&5, &3);
        assert_eq!(result, 2);
    }

    #[test]
    fn can_calculate_absolute_difference_with_negative_number() {
        let result = absolute_difference(&-5, &3);
        assert_eq!(result, 8);
    }

    #[test]
    fn can_calculate_absolute_difference_with_zero() {
        let result = absolute_difference(&0, &3);
        assert_eq!(result, 3);
    }

    #[test]
    fn can_calculate_absolute_difference_with_large_number() {
        let result = absolute_difference(&1234567890, &3);
        assert_eq!(result, 1234567887);
    }

    #[test]
    fn can_calculate_absolute_difference_with_large_negative_number() {
        let result = absolute_difference(&-1234567890, &3);
        assert_eq!(result, 1234567893);
    }
}
