pub fn is_leap_year(year: u32) -> bool {
    if (year % 4) == 0 {
        true
    } else {
        false
    }
}
