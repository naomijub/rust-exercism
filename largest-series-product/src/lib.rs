#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    let mut digits: Vec<u64> = string_digits
            .split("")
            .map(|s| s.parse())
            .filter(|s| match s {
                Ok(_) => true,
                Err(_) => false,
            })
            .map(|s| s.unwrap())
            .collect();
    digits.sort();
    digits.reverse();
    digits.truncate(span);

    Ok(digits.iter().fold(1,|acc, x| acc * x))
}
