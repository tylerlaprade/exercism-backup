#[must_use]
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut open_pairs = Vec::new();
    string.chars().all(|c| match c {
        '(' | '[' | '{' => {
            open_pairs.push(c);
            true
        }
        ')' => open_pairs.pop().unwrap_or(' ') == '(',
        ']' => open_pairs.pop().unwrap_or(' ') == '[',
        '}' => open_pairs.pop().unwrap_or(' ') == '{',
        _ => true,
    }) && open_pairs.is_empty()
}
