#[must_use]
pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    diagram
        .split('\n')
        .flat_map(|line| {
            line.chars()
                .skip(((student.bytes().next().unwrap() - b'A') * 2).into())
                .take(2)
                .map(|c| match c {
                    'V' => "violets",
                    'R' => "radishes",
                    'C' => "clover",
                    'G' => "grass",
                    _ => unreachable!(),
                })
        })
        .collect()
}
