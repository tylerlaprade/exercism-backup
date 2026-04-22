use std::collections::HashSet;

#[must_use]
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        // I'm not sure exactly why I needed all these &'s, but they made the compiler happy
        .filter(|&&factor| factor > 0)
        .flat_map(|&factor| (factor..limit).step_by(factor as usize))
        .collect::<HashSet<u32>>()
        .iter()
        .sum::<u32>()
}
