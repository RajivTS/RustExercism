#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sublist<T: PartialEq>(small_list: &[T], big_list: &[T]) -> bool {
    let window_size = small_list.len();
    return big_list.windows(window_size).any(|window| window == small_list);
}

fn handle_empty<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() == 0 && _second_list.len() == 0 {
        return Comparison::Equal;
    } else if _first_list.len() == 0 {
        return Comparison::Sublist;
    } else if _second_list.len() == 0 {
        return Comparison::Superlist;
    } else {
        return Comparison::Unequal;
    }
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let equal_len = _first_list.len() == _second_list.len();
    if _first_list.len() == 0 || _second_list.len() == 0 {
        return handle_empty(_first_list, _second_list);
    } else if is_sublist(_first_list, _second_list) {
        return if equal_len { Comparison::Equal } else { Comparison::Sublist };
    } else if is_sublist(_second_list, _first_list) {
        return Comparison::Superlist;
    }
    return Comparison::Unequal;
}
