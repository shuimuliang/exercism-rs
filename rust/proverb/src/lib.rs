pub fn build_proverb(list: &[&str]) -> String {
    let mut res  = String::new();
    if list.is_empty()  {
        return res
    }

    for pair in list.windows(2) {
        if let [first, second] = pair {
            res += &format!("For want of a {} the {} was lost.\n", first, second)
        }
    }
    res += &format!("And all for the want of a {}.", list[0]);
    res
}
