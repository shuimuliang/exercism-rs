pub fn reply(message: &str) -> &str {
    // 移除额外的空格
    let cleaned_message = message.trim();

    let isUpperfn = |s: &str| s.to_uppercase() == s;

    let isAlpha = cleaned_message.chars().any(|x| x.is_alphabetic());
    let isUpper = isUpperfn(cleaned_message) && isAlpha;
    let isQuestion = cleaned_message.ends_with('?');

    if cleaned_message.is_empty() {
        return "Fine. Be that way!"
    }

    if isUpper {
        if isQuestion {
            return "Calm down, I know what I'm doing!"
        } else {
            return "Whoa, chill out!"
        }
    } else {
        if isQuestion {
            return "Sure."
        } else {
            return "Whatever."
        }
    }
}
