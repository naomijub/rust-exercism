/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.trim().len() <= 1 {
        return false;
    }

    code
        .replace(" ", "")
        .chars()
        .rev()
        .map(|ch| ch.to_digit(10))
        .enumerate()
        .try_fold(0u32, |acc, (i, n)| 
            match (i % 2, n) {
                (0, Some(v)) => Some(acc + v),
                (1, Some(v)) if v == 9 => Some(acc + v),
                (1, Some(v)) => Some(acc + ((v * 2) % 9)),
                _ => None
            })
        .map_or(false, |v| v % 10 == 0)
}
