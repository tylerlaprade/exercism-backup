#[must_use]
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield.iter().enumerate().map(|(j, row)| {
        row.as_bytes().iter().enumerate().map(|(i, square)| {
            if *square == b'*' {
                '*'
            } else {
                match minefield.iter()
                    .take(minefield.len().min(j + 2))
                    .skip(j.saturating_sub(1))
                    .map(|neighbor_row| {
                        neighbor_row.as_bytes().iter()
                            .take(minefield[0].len().min(i + 2))
                            .skip(i.saturating_sub(1))
                            .filter(|neighbor| **neighbor == b'*')
                            .count()
                    })
                    .sum() {
                    0 => ' ',
                    neighboring_bomb => char::from_digit(neighboring_bomb as u32, 10)
                        .expect("We know it has to be a single digit because a square only has 8 neighbors."),
                }
            }
        }).collect()
    }).collect()
}
