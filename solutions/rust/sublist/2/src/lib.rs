#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let (comparison, shorter, longer) =
        match (_first_list.len() as i32 - _second_list.len() as i32).signum() {
            0 => {
                if _first_list == _second_list {
                    return Comparison::Equal;
                } else {
                    return Comparison::Unequal;
                }
            }
            -1 => (Comparison::Sublist, _first_list, _second_list),
            1 => (Comparison::Superlist, _second_list, _first_list),
            _ => panic!("Invalid signum"),
        };
    if shorter.is_empty() {
        return comparison;
    }
    for window in longer.windows(shorter.len()) {
        if window == shorter {
            return comparison;
        }
    }
    Comparison::Unequal
}
