#[must_use]
pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(&[' ', '-'])
        .filter_map(|word| word.chars().find(|arg0: &char| char::is_alphabetic(*arg0)))
        .map(|c| c.to_ascii_uppercase())
        .collect::<String>()
}
