use std::collections::{HashMap, HashSet};

#[must_use]
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut parts = input.split(" == ");
    let addends = parts.next()?.split(" + ");
    let sum = parts.next()?;
    let mut nonzero = HashSet::new();
    let mut equations: Vec<HashMap<&u8, i8>> = Vec::new();
    for addend in addends {
        let bytes = addend.as_bytes();
        nonzero.insert(bytes[0]);
        for (i, letter) in bytes.iter().enumerate().rev() {
            if equations.len() == i {
                equations.push(HashMap::new());
            }
            equations[bytes.len() - 1 - i]
                .entry(letter)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
    }
    let mut carries: Vec<i8> = equations
        .iter()
        .map(|equation| equation.iter().map(|(&_, &count)| count).sum::<i8>() * 9 / 10)
        .collect();
    let bytes = sum.as_bytes();
    nonzero.insert(bytes[0]);
    for (i, letter) in bytes.iter().enumerate().rev() {
        if equations.len() == i {
            equations.push(HashMap::new());
        }
        equations[bytes.len() - 1 - i]
            .entry(letter)
            .and_modify(|e| *e -= 1)
            .or_insert(-1);
    }
    let mut letter_possibilities: HashMap<u8, HashSet<u8>> = HashMap::new();
    input
        .bytes()
        .filter(u8::is_ascii_uppercase)
        .for_each(|byte| {
            letter_possibilities
                .entry(byte)
                .or_insert((u8::from(nonzero.contains(&byte))..10).collect());
        });
    let mut number_possibilities: HashMap<u8, HashSet<u8>> = HashMap::new();
    for x in 0..10 {
        number_possibilities.entry(x).or_default().extend(
            letter_possibilities
                .keys()
                .filter(|&&byte| x != 0 || !nonzero.contains(&byte))
                .copied(),
        );
    }
    let solution = HashMap::new();

    None
}
