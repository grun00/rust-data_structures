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

    if v.len() <= 1 {
        return v;
    }

    let mut result: Vec<T> = Vec::with_capacity(v.len());
    let last_half = v.split_off(v.len() / 2);
    let first_half = merge_sort(v);
    let last_half = merge_sort(last_half);

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

pub fn quick_sort<T: PartialOrd + Debug>(v: &mut [T]){
    if v.len() <= 1{
        return;
    }

    let p = pivot(v);
    println!("{:?}", v);

    let (a, b) = v.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]);
}

pub fn pivot<T: PartialOrd>(v: &mut [T]) -> usize {
    let mut p = 0;
    for i in 1..v.len() {
        if v[i] < v[p]{
            v.swap(p, p + 1);
            p+=1;
        }
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_test() {
        let mut v = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        buble_sort(&mut v);
        assert_eq!(v, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
    #[test]
    fn merge_sort_test() {
        let mut v = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        v = merge_sort(v);
        assert_eq!(v, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
    #[test]
    fn test_pivot() {
        let mut v = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let p = pivot(&mut v);

        for i in 0..v.len() {
            assert!(( v[i] < v[p] ) == ( i < p ));
        }
    }
    #[test]
    fn quick_sort_test() {
        let mut v = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        quick_sort(&mut v);
        assert_eq!(v, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
