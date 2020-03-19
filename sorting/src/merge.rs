pub fn sort(data: &[u32], comparator: impl Fn(i32, i32) -> i32) -> (Vec<u32>, u32, u32) {
    let mut out = data.to_vec();
    let length = out.len() - 1;
    let (comparisons, swaps) = _sort_(&mut out, 0, length, &comparator);

    (out, comparisons, swaps)
}

fn _sort_(
    data: &mut Vec<u32>,
    start: usize,
    end: usize,
    comparator: &impl Fn(i32, i32) -> i32,
) -> (u32, u32) {
    let mut comparisons = 0u32;
    let mut swaps = 0u32;

    if start < end {
        let mid = (start + end) >> 1;
        let mut results: (u32, u32);

        results = _sort_(data, start, mid, comparator);
        comparisons += results.0;
        swaps += results.1;

        results = _sort_(data, mid + 1, end, comparator);
        comparisons += results.0;
        swaps += results.1;

        results = _merge_(data, start, mid, end, comparator);
        comparisons += results.0;
        swaps += results.1;
    }

    (comparisons, swaps)
}

fn _merge_(
    data: &mut Vec<u32>,
    start: usize,
    mid: usize,
    end: usize,
    comparator: &impl Fn(i32, i32) -> i32,
) -> (u32, u32) {
    let mut left: Vec<i32> = data[start..=mid].iter().map(|&val| val as i32).collect();
    let mut right: Vec<i32> = data[(mid + 1)..=end]
        .iter()
        .map(|&val| val as i32)
        .collect();

    left.push(-1);
    right.push(-1);

    let mut comparisons = 0u32;
    let mut swaps = 0u32;

    let mut i = 0;
    let mut j = 0;
    for index in start..=end {
        if comparator(left[i], right[j]) < 0 {
            data[index] = left[i] as u32;
            i += 1;
            comparisons += 1;
        } else {
            data[index] = right[j] as u32;
            j += 1;
            comparisons += 1;
            swaps += 1;
        }
    }
    (comparisons, swaps)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mergesort_ordered() {
        let unordered: Vec<u32> = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let ordered = sort(&unordered, |x, y| y - x);
        assert_eq!(ordered.0, unordered);
    }
    #[test]
    fn test_mergesort_antiordered() {
        let unordered: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let ordered = sort(&unordered, |x, y| y - x);
        assert_eq!(ordered.0, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_mergesort_random() {
        let unordered: Vec<u32> = vec![15, 4, 8, 5, 16, 1, 21, 3, 30, 6];
        let ordered = sort(&unordered, |x, y| y - x);
        assert_eq!(ordered.0, vec![30, 21, 16, 15, 8, 6, 5, 4, 3, 1]);
    }
    #[test]
    fn test_mergesort_a_shape() {
        let unordered: Vec<u32> = vec![2, 4, 6, 8, 10, 9, 7, 5, 3, 1];
        let ordered = sort(&unordered, |x, y| y - x);
        assert_eq!(ordered.0, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }
    #[test]
    fn test_mergesort_v_shape() {
        let unordered: Vec<u32> = vec![9, 7, 5, 3, 1, 2, 4, 6, 8, 10];
        let ordered = sort(&unordered, |x, y| y - x);
        assert_eq!(ordered.0, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }
}
