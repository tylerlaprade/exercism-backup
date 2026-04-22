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
    if shorter.len() == 0 {
        return comparison;
    }
    let mut indices = Vec::new();
    for item in longer {
        let mut new_indices = Vec::new();
        indices.push(0);
        for index in indices {
            if &shorter[index] == item {
                if index == shorter.len() - 1 {
                    return comparison;
                }
                new_indices.push(index + 1)
            }
        }
        indices = new_indices;
    }
    Comparison::Unequal
}
