pub fn raindrops(n: u32) -> String {
    let mut res = String::new();
    let mut hit = false;
    if n % 3 == 0 {
       res.push_str("Pling");
       hit = true;
    }
    if n % 5 == 0 {
        res.push_str("Plang");
        hit = true;
    }
    if n % 7 == 0 {
        res.push_str("Plong");
        hit = true;
    }

    match hit {
        true => res,
        false => n.to_string(),
    }
}
