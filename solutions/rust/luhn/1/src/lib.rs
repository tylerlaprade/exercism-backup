#[must_use]
pub fn is_valid(code: &str) -> bool {
    let digits = code
        .chars()
        .filter(|c| c != &' ')
        .collect::<Vec<char>>()
        .into_iter();
    let len = digits.len();
    if len < 2 {
        return false;
    }
    let mut needs_double = len % 2 == 0;
    let mut running_sum = 0;
    for digit in digits {
        let Some(mut digit) = digit.to_digit(10) else {
            return false;
        };
        if needs_double {
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
        needs_double = !needs_double;
    }
    running_sum == 0
}
