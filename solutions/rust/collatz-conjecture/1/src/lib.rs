#[must_use]
pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut i = 0;
    let mut number = n;
    while number != 1 {
        match number % 2 {
            0 => number /= 2,
            1 => number = 3 * number + 1,
            _ => unreachable!(),
        }
        i += 1;
    }
    Some(i)
}
