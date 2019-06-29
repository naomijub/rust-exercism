#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(String),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() { return Err(Error::SpanTooLong); }
    else if span == 0 { return Ok(1u64); }
    else if string_digits.matches(char::is_alphabetic).collect::<Vec<&str>>().len() > 0 {
        let digit = string_digits.matches(char::is_alphabetic).collect::<Vec<&str>>();
        return Err(Error::InvalidDigit(digit.first().unwrap().to_owned().to_string()));
    }

    let digits: Vec<u64> = string_digits
            .split("")
            .map(|s| s.parse())
            .filter(|s| match s {
                Ok(_) => true,
                Err(_) => false,
            })
            .map(|s| s.unwrap())
            .collect();

    Ok(str_chunks(digits, span)
            .first()
            .unwrap()
            .to_owned())
}

fn str_chunks(digits: Vec<u64>, span: usize) -> Vec<u64> {
    let mut str_chunks = digits
        .windows(span)
        .map(|x| x.to_vec())
        .map(|i| i.iter().fold(1,|acc, x| acc * x))
        .collect::<Vec<u64>>();
    str_chunks.sort();
    str_chunks.reverse();
    str_chunks

}