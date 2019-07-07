const RADIX: u32 = 10;

pub fn is_armstrong_number(num: u32) -> bool {
    let pow = num.to_string().len() as u32;
    num.to_string()
      .chars()
      .into_iter()
      .map(|c| c.to_digit(RADIX).unwrap())
      .map(|n| n.pow(pow))
      .fold(0, |acc, n| acc + n) == num
}
