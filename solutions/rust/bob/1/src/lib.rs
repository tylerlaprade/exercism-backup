#[must_use]
pub fn reply(message: &str) -> &str {
    if message
        .chars()
        .all(|arg0: char| char::is_ascii_whitespace(&arg0))
    {
        return "Fine. Be that way!";
    }
    let is_question = message.trim_ascii_end().ends_with('?');
    if message
        .chars()
        .all(|arg0: char| !char::is_ascii_lowercase(&arg0))
        && message
            .chars()
            .any(|arg0: char| char::is_ascii_uppercase(&arg0))
    {
        if is_question {
            "Calm down, I know what I'm doing!"
        } else {
            "Whoa, chill out!"
        }
    } else if is_question {
        "Sure."
    } else {
        "Whatever."
    }
}
