#[must_use]
pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    num_str
        .bytes()
        .map(|b| u32::from(b - b'0').pow(num_str.len() as u32))
        .sum::<u32>()
        == num
}
