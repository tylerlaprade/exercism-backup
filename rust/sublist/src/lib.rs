use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let (comparison, shorter, longer) = match first_list.len().cmp(&second_list.len()) {
        Ordering::Equal => {
            return if first_list == second_list {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
        Ordering::Less => (Comparison::Sublist, first_list, second_list),
        Ordering::Greater => (Comparison::Superlist, second_list, first_list),
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
