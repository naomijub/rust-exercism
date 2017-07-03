pub fn is_leap_year(year: u32) -> bool {
    let is_mult = |n| year % n == 0;
    is_mult(4) && (is_mult(400) || !is_mult(100))
}
