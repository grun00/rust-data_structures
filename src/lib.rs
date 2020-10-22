use rand::Rng;
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

pub fn quick_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    let (a, b) = v.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]);
}
#[allow(dead_code)]
struct RawSend<T>(*mut [T]);
#[allow(dead_code)]
unsafe impl<T> Send for RawSend<T> {}

#[allow(dead_code)]
pub fn threaded_quick_sort<T: 'static + PartialOrd + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let p = pivot(v);

    let (a, b) = v.split_at_mut(p);


    let raw_a: *mut [T] = a as *mut [T];
    let raw_s = RawSend(raw_a);

    unsafe {
        let handle = std::thread::spawn(move || {
            threaded_quick_sort(&mut *raw_s.0);
        });

        threaded_quick_sort(&mut b[1..]);

        handle.join().ok();
    }
}

pub fn rayon_quick_sort<T: Send + PartialOrd>(v: &mut [T]){
    if v.len() <=1 {
        return;
    }
    let p = pivot(v);

    let (a, b) = v.split_at_mut(p);

    rayon::join(|| rayon_quick_sort(a), || rayon_quick_sort(&mut b[1..]));
}


pub fn pivot<T: PartialOrd>(v: &mut [T]) -> usize {
    let mut rng = rand::thread_rng();
    let mut p = rng.gen_range(0, v.len());
    v.swap(0, p);
    p = 0;

    for i in 1..v.len() {
        if v[i] < v[p] {
            v.swap(p + 1, i);
            v.swap(p, p + 1);
            p += 1;
        }
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn avg_case_bubble_sort_test() {
        let mut rng = rand::thread_rng();
        let mut v: Vec<u64> = (0..1000).map(|_| rng.gen_range(0, 100)).collect();
        buble_sort(&mut v);
        for i in 0..v.len() - 1 {
            assert!(v[i] <= v[i + 1]);
        }
    }
    #[test]
    fn best_case_bubble_sort_test() {
        let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        buble_sort(&mut v);
        assert_eq!(v, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
    #[test]
    fn avg_case_merge_sort_test() {
        let mut rng = rand::thread_rng();
        let mut v: Vec<u64> = (0..1000).map(|_| rng.gen_range(0, 100)).collect();
        v = merge_sort(v);
        for i in 0..v.len() - 1 {
            assert!(v[i] <= v[i + 1]);
        }
    }
    #[test]
    fn best_case_merge_sort_test() {
        let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        v = merge_sort(v);
        assert_eq!(v, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
    #[test]
    fn test_pivot() {
        let mut v = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let p = pivot(&mut v);

        for i in 0..v.len() {
            assert!((v[i] < v[p]) == (i < p));
        }
    }
    #[test]
    fn avg_case_quick_sort_test() {
        let mut rng = rand::thread_rng();
        let mut v: Vec<u64> = (0..1000).map(|_| rng.gen_range(0, 100)).collect();
        quick_sort(&mut v);
        for i in 0..v.len() - 1 {
            assert!(v[i] <= v[i + 1]);
        }
    }
    #[test]
    fn best_case_quick_sort_test() {
        let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        quick_sort(&mut v);
        assert_eq!(v, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
    #[test]
    fn avg_case_unsafe_threaded_quick_sort_test() {
        let mut rng = rand::thread_rng();
        let mut v: Vec<u64> = (0..1000).map(|_| rng.gen_range(0, 100)).collect();
        threaded_quick_sort(&mut v);
        for i in 0..v.len() - 1 {
            assert!(v[i] <= v[i + 1]);
        }
    }
    #[test]
    fn avg_case_rayon_threaded_quick_sort_test() {
        let mut rng = rand::thread_rng();
        let mut v: Vec<u64> = (0..1000).map(|_| rng.gen_range(0, 100)).collect();
        rayon_quick_sort(&mut v);
        for i in 0..v.len() - 1 {
            assert!(v[i] <= v[i + 1]);
        }
    }
}
