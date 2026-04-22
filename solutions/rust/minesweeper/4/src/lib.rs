#[must_use]
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len();
    if height == 0 {
        return vec![];
    }
    let width = minefield[0].len();
    minefield.iter().enumerate().map(|(j, row)| {
        row.as_bytes().iter().enumerate().map(|(i, square)| {
            if *square == b'*' {
                '*'
            } else {
                let neighboring_bombs =
                    minefield.iter().take(height.min(j + 2)).skip(j.saturating_sub(1)).flat_map(|neighbor_row| {
                        neighbor_row.as_bytes().iter().take(width.min(i + 2)).skip(i.saturating_sub(1))
                    }).filter(|neighbor| **neighbor == b'*').count();
                if neighboring_bombs == 0 {
                    ' '
                } else {
                    char::from_digit(neighboring_bombs as u32, 10).expect("We know it has to be a single digit because a square only has 8 neighbors.")
                }
            }
        }).collect()
    }).collect()
}
