#[must_use]
pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut num = n;
    let mut divisor = 2;

    while num > 1 {
        // I'm surprised this runs so fast even for huge numbers since we have to check
        // division for every single number. Maybe the compiler optimizes it somehow?
        while num % divisor == 0 {
            factors.push(divisor);
            num /= divisor;
        }
        divisor += 1;
    }

    factors
}
