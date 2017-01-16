pub fn powerset<T>(els: &[T]) -> Vec<Vec<T>>
    where T: Copy
{
    let size = 2usize.pow(els.len() as u32);
    let mut powerset = Vec::with_capacity(size);

    for mask in 0..size {
        let mut set = vec![];
        for idx in 0..els.len() {
            if mask & (1 << idx) > 0 {
                set.push(els[idx]);
            }
        }
        powerset.push(set);
    }

    powerset
}

#[cfg(test)]
mod tests {
    use super::powerset;


    #[test]
    fn test_empty_set() {
        let s: Vec<i32> = Vec::new();
        let p = powerset(&s[..]);
        assert_eq!(p, vec![vec![]]);
    }

    #[test]
    fn test_1_set() {
        let s = vec![1];
        let p = powerset(&s[..]);
        assert_eq!(p, vec![
            vec![],
            vec![1],
        ]);
    }

    #[test]
    fn test_2_set() {
        let s = vec![1, 2];
        let p = powerset(&s[..]);
        assert_eq!(p, vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
        ]);
    }

    #[test]
    fn test_3_set() {
        let s = vec![1, 2, 3];
        let p = powerset(&s[..]);
        assert_eq!(p, vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ]);
    }
}
