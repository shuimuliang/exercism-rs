pub fn is_armstrong_number(num: u32) -> bool {

    let powcount = pow_count(num);
    let mut sum = 0;
    for i in 1..=powcount {
        let bitmum = num / 10u32.pow(powcount-i) % 10;
        sum += bitmum.pow(powcount);
    }
    if sum == num {true} else {false}
}

fn pow_count(n: u32) -> u32 {
    let mut curr = n;
    let mut count = 0;
    loop {
        curr = curr / 10;
        count = count + 1;

        if curr == 0 {
            break;
        }
    }
    count
}

