const INPUT: std::ops::RangeInclusive<u32> = (158_126..=624_574);

fn main() {
    println!(
        "{}",
        INPUT
            .filter(|&n| adjacent_digits_identical(n))
            .filter(|&n| !digits_decrease(n))
            .count()
    );
}

fn adjacent_digits_identical(n: u32) -> bool {
    let n = n.to_string();
    n.chars().zip(n.chars().skip(1)).any(|(a, b)| a == b)
}

fn digits_decrease(n: u32) -> bool {
    let n = n.to_string();
    n.chars().zip(n.chars().skip(1)).any(|(a, b)| a > b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjacent() {
        assert_eq!(adjacent_digits_identical(5), false);
        assert_eq!(adjacent_digits_identical(11), true);
        assert_eq!(adjacent_digits_identical(12_33_12), true);
        assert_eq!(adjacent_digits_identical(9999), true);
        assert_eq!(adjacent_digits_identical(101010), false);
    }

    #[test]
    fn test_non_decreasing() {
        assert_eq!(digits_decrease(1234), false);
        assert_eq!(digits_decrease(4321), true);
        assert_eq!(digits_decrease(1111), false);
        assert_eq!(digits_decrease(1), false);
        assert_eq!(digits_decrease(0), false);
    }
}
