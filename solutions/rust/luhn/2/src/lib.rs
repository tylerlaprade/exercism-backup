#[must_use]
pub fn is_valid(code: &str) -> bool {
    let digits = code
        .chars()
        .filter(|c| c != &' ')
        .collect::<Vec<char>>()
        .into_iter();
    if digits.len() < 2 {
        return false;
    }
    let mut running_sum = 0;
    for (i, digit) in digits.rev().enumerate() {
        let Some(mut digit) = digit.to_digit(10) else {
            return false;
        };
        if i % 2 == 1 {
            digit = match digit {
                0 => 0,
                1 => 2,
                2 => 4,
                3 => 6,
                4 => 8,
                5 => 1,
                6 => 3,
                7 => 5,
                8 => 7,
                9 => 9,
                _ => unreachable!("Only a single digit"),
            }
        }
        running_sum = (running_sum + digit) % 10;
    }
    running_sum == 0
}
