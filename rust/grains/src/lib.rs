pub fn square(s: u32) -> u64 {
    if s == 0 {
        panic!("Square must be between 1 and 64")
    } else if s > 64 {
        panic!("Square must be between 1 and 64")
    }
    return (2 as u64).pow(s-1);
}

pub fn total() -> u64 {
    let sum = (1..=64).map(|x| 2u64.pow(x-1)).sum();
    sum
}
