// O(n^2)
pub fn buble_sort<T:PartialOrd>(v: &mut [T]){
    for p in 0..v.len() {
        let mut sorted = true;
        for i in 0..v.len() - 1 - p {
            if v[i] > v[i + 1]{
                v.swap(i, i + 1);
                sorted = false;
            }
        }

        if sorted {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_sorts_array() {
        let mut v = vec![5, 4, 3, 2, 1];
        buble_sort(&mut v);
        assert_eq!(v, [1, 2, 3, 4, 5]);
    }
}
