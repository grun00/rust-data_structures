use std::fmt::Debug;

// O(n^2)
pub fn buble_sort<T: PartialOrd>(v: &mut [T]) {
    for p in 0..v.len() {
        let mut sorted = true;
        for i in 0..v.len() - 1 - p {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        if sorted {
            return;
        }
    }
}

// O(n*ln(n))
pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {

    println!("MS:{:?}", v);
    if v.len() <= 1 {
        return v;
    }

    let mut result: Vec<T> = Vec::with_capacity(v.len());
    let mut last_half = v.split_off(v.len() / 2);
    let first_half = merge_sort(v);
    last_half = merge_sort(last_half);

    let mut first_half_iterator = first_half.into_iter();
    let mut last_half_iterator = last_half.into_iter();
    let mut first_half_peek = first_half_iterator.next();
    let mut last_half_peek = last_half_iterator.next();

    loop {
        match first_half_peek {
            Some(ref first_half_value) => match last_half_peek {
                Some(ref last_half_value) => {
                    if last_half_value < first_half_value {
                        result.push(last_half_peek.take().unwrap());
                        last_half_peek = last_half_iterator.next();
                    } else {
                        result.push(first_half_peek.take().unwrap());
                        first_half_peek = first_half_iterator.next();
                    }
                }
                None => {
                    result.push(first_half_peek.take().unwrap());
                    result.extend(first_half_iterator);
                    return result;
                }
            },
            None => {
                if let Some(last_half_value) = last_half_peek {
                    result.push(last_half_value);
                }
                result.extend(last_half_iterator);
                return result;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bubble_sort_array() {
        let mut v = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        buble_sort(&mut v);
        assert_eq!(v, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
    #[test]
    fn merge_sort_array() {
        let mut v = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        v = merge_sort(v);
        assert_eq!(v, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
