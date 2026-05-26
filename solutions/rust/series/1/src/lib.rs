use std::collections::VecDeque;

#[must_use]
pub fn series(digits: &str, len: usize) -> Vec<String> {
    if digits.is_empty() || len > digits.len() {
        return Vec::new();
    }
    let mut binding = digits.chars();
    let digits = binding.by_ref();
    let mut current_chars = digits.take(len).collect::<VecDeque<_>>();
    let mut out_vec = Vec::from([current_chars.iter().collect::<String>()]);
    for digit in digits {
        current_chars.pop_front();
        current_chars.push_back(digit);
        out_vec.push(current_chars.iter().collect::<String>());
    }
    out_vec
}
