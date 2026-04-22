#[must_use]
pub fn square(s: u32) -> u64 {
    2_u64.pow(s - 1)
}

#[must_use]
pub fn total() -> u64 {
    u64::MAX
}
