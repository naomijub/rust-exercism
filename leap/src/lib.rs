pub fn is_leap_year(year: u32) -> bool {
    is_multiple(year, 4) && (is_multiple(year, 400) || !is_multiple(year, 100))
}

fn is_multiple(year: u32, num: u32) -> bool {
    year % num == 0
}
