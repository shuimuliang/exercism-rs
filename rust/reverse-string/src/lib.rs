pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

pub fn reverse2(input: &str) -> String {
    let svec = String::from(input);
    let mut output= String::new();
    for t in svec.chars().rev()  {
        output.push(t);
    }
    output
}

