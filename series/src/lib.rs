pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len <= 0 { return vec!["".to_string(); digits.len() + 1]; }
    digits.chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .windows(len)
        .map(|v| v.join(""))
        .collect::<Vec<String>>()
}
