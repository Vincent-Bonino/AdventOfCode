pub fn power<T: Copy>(elements: &[T], length: usize) -> Vec<Vec<T>> {
    _power(elements, vec![], length)
}

fn _power<T: Copy>(elements: &[T], partial_result: Vec<T>, depth: usize) -> Vec<Vec<T>> {
    if depth == 0 {
        return vec![partial_result];
    }

    let mut result: Vec<Vec<T>> = Vec::new();

    for elmt in elements {
        let mut with_elmt: Vec<T> = partial_result.clone();
        with_elmt.push(*elmt);

        result.extend(_power(elements, with_elmt, depth - 1))
    }

    result
}
