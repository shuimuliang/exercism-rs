pub fn is_leap_year(year: u64) -> bool {
    // 能被400整除的,或者不能被100整除而能被4整除的年就是闰年,
    if year % 400 == 0 {
        true
    } else if year % 100 != 0 {
        if year % 4 == 0 {
            true
        } else {
            false
        }
    } else {
        false
    }
}
