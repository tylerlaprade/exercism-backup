#[must_use]
pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }
    let start = list[0];
    let mut prev = start;
    // I tried to do this in a functional way. Is this a good approach?
    list.iter()
        .skip(1)
        .map(|word| {
            let ret = format!("For want of a {prev} the {word} was lost.\n").to_string();
            prev = word;
            ret
        })
        .collect::<String>()
        + &format!("And all for the want of a {start}.")
}
