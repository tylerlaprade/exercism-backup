use std::collections::BTreeMap;

#[must_use]
pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter().flat_map(|tuple| {
        tuple.1.iter().map(|letter| (letter.to_ascii_lowercase(), *tuple.0))
    }).collect()
}
