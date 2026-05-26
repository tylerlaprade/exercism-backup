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
                for y in j.saturating_sub(1)..height.min(j + 2) {
                    for x in i.saturating_sub(1)..width.min(i + 2) {
                        if minefield[y].as_bytes()[x] != b'*' {
                            result[y][x] += 1;
                        }
                    }
                }
            }
        }
    }
    result.iter().enumerate().map(|(j, row)| {
        row.iter().enumerate().map(|(i, &c)| {
            if minefield[j].as_bytes()[i] == b'*' {
                '*'
            } else if c == 0 {
                ' '
            } else {
                char::from_digit(c, 10).expect("We know it has to be a single digit because a square only has 8 neighbors.")
            }
        }).collect()
    }).collect()
}
