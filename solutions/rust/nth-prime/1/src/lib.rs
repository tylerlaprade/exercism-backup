#[allow(clippy::cast_possible_truncation)]
#[must_use]
pub fn nth(n: u32) -> u32 {
    let mut sieve = vec![true; 104_744].into_boxed_slice();

    let mut i = 2;
    for _ in 0..n {
        for x in ((i * i)..sieve.len()).step_by(i) {
            sieve[x] = false;
        }
        i += 1;
        while !sieve[i] {
            i += 1;
        }
    }
    i as u32
}
