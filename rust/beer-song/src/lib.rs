pub fn verse(n: i32) -> String {
    format!("{} of beer on the wall, {} of beer.\n{}, {} of beer on the wall.\n",
            bottle_count(n, true),
            bottle_count(n, false),
            action(n),
            bottle_count(n - 1, false))
}

fn bottle_count(n: i32, upper: bool) -> String {
    match n {
        -1 => "99 bottles".into(),
        0 if upper => "No more bottles".into(),
        0  => "no more bottles".into(),
        1 => format!("{} bottle", n).into(),
        _ => format!("{} bottles", n).into(),
    }
}

fn action(n: i32) -> String {
    match n {
        0 => "Go to the store and buy some more".into(),
        1 => "Take it down and pass it around".into(),
        _ => "Take one down and pass it around".into(),
    }
}

pub fn sing(start: i32, end: i32) -> String {
    let mut res: Vec<String> = Vec::new();
    for i in (end..=start).rev() {
        res.push(verse(i));
    }
    res.join("\n")
}
