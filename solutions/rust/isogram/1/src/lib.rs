use std::collections::HashSet;

#[must_use]
pub fn check(candidate: &str) -> bool {
    let mut seen = HashSet::new();
    candidate
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .all(|c| {
            if seen.contains(&c) {
                false
            } else {
                seen.insert(c);
                true
            }
        })
}
