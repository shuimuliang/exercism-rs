pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    // brackets [], braces {}, parentheses (),

    for c in string.chars() {
        let topchar = stack.last().cloned();
        match c {
            '[' | '{' | '(' => stack.push(c),
            ']' => {
               if topchar == Some('[') {
                  stack.pop();
               }
                else {
                    return false;
                }
            }
            '}' => {
                if topchar == Some('{') {
                    stack.pop();
                }
                else {
                    return false;
                }
            }
            ')' => {
                if topchar == Some('(') {
                    stack.pop();
                }
                else {
                    return false;
                }
            }
            _ => (),
        }
    }
    stack.is_empty()
}
