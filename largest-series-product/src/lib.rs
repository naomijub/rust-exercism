#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
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