#[must_use]
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len();
    if height == 0 {
        return vec![];
    }
    let width = minefield[0].len();
    let mut result = vec![vec![0; width]; height];
    for (j, row) in minefield.iter().enumerate() {
        for (i, square) in row.as_bytes().iter().enumerate() {
            if *square == b'*' {
                for y in if j == 0 { j } else { j - 1 }..=if j + 1 == height { j } else { j + 1 } {
                    for x in if i == 0 { i } else { i - 1 }..=if i + 1 == width { i } else { i + 1 } {
                        if minefield[y].as_bytes()[x] != b'*' {
                            result[y][x] += 1;
                        }
                    }
                }
            }
        }
    }
    result
        .iter()
        .enumerate()
        .map(|(j, row)| {
            row.iter()
                .enumerate()
                .map(|(i, &c)| {
                    if minefield[j].as_bytes()[i] == b'*' {
                        "*".to_string()
                    } else if c == 0 {
                        " ".to_string()
                    } else {
                        c.to_string()
                    }
                })
                .collect()
        })
        .collect()
}
