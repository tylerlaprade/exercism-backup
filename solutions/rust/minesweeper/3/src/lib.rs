#[must_use]
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len();
    if height == 0 {
        return vec![];
    }
    let width = minefield[0].len();
    minefield.iter().enumerate().map(|(j, row)| {
        row.as_bytes().iter().enumerate().map(|(i, square)| {
            if *square == b'*' { '*' } else {
                let mut neighboring_bombs = 0;
                for y in j.saturating_sub(1)..height.min(j + 2) {
                    for x in i.saturating_sub(1)..width.min(i + 2) {
                        if minefield[y].as_bytes()[x] == b'*' {
                            neighboring_bombs += 1;
                        }
                    }
                }
                if neighboring_bombs == 0 {
                    ' '
                } else {
                    char::from_digit(neighboring_bombs, 10).expect("We know it has to be a single digit because a square only has 8 neighbors.")
                }
            }
        }).collect()
    }).collect()
}
