fn sum_of_first(n: u32) -> u32 {
    n * (n + 1) / 2
}

#[must_use]
pub fn square_of_sum(n: u32) -> u32 {
    sum_of_first(n).pow(2)
}

#[must_use]
pub fn sum_of_squares(n: u32) -> u32 {
    sum_of_first(n) * (2 * n + 1) / 3
}

#[must_use] 
pub fn difference(n: u32) -> u32 {
    let sum_of_first_n = sum_of_first(n);
    sum_of_first_n * (3 * sum_of_first_n - (2 * n + 1)) / 3
}
