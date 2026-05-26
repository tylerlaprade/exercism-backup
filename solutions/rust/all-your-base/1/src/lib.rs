#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }
    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }
    if number.iter().any(|&digit| digit >= from_base) {
        return Err(Error::InvalidDigit(
            // Why did I need * and && here?
            *number.iter().find(|&&digit| digit >= from_base).unwrap(),
        ));
    }
    let mut place_value = 1;

    let mut number = number
        .iter()
        .rev()
        .map(|digit| {
            // I tried to validate the digit in here but couldn't figure out how to early return the error
            let product = digit * place_value;
            place_value *= from_base;
            product
        })
        .sum();
    let mut new_number = Vec::new();
    let mut place_values = Vec::from([1]);
    let mut place_value = to_base;
    while place_value <= number {
        place_values.push(place_value);
        place_value *= to_base;
    }
    for place_value in place_values.into_iter().rev() {
        new_number.push(number / place_value);
        number %= place_value;
    }
    Ok(new_number)
}
