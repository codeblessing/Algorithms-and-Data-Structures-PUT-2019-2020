pub fn sort(data: &Vec<i32>, comparator: impl Fn(&i32, &i32) -> i32) -> (Vec<i32>, u32, u32) {
    let mut out = data.clone();
    let length = out.len() - 1;
    let (comparisons, swaps) = _sort_(&mut out, 0, length, &comparator);

    (out, comparisons, swaps)
}

fn _sort_(
    data: &mut Vec<i32>,
    start: usize,
    end: usize,
    comparator: &impl Fn(&i32, &i32) -> i32,
) -> (u32, u32) {
    let mut comparisons = 0u32;
    let mut swaps = 0u32;

    if start < end {
        let mid = (start + end) / 2;
        let (comps, swps) = _sort_(data, start, mid, comparator);
        comparisons += comps;
        swaps += swps;
        let (comps, swps) = _sort_(data, mid + 1, end, comparator);
        comparisons += comps;
        swaps += swps;
        let (comps, swps) = _merge_(data, start, mid, end, comparator);
        comparisons += comps;
        swaps += swps;
    }

    (comparisons, swaps)
}

fn _merge_(
    data: &mut Vec<i32>,
    start: usize,
    mid: usize,
    end: usize,
    comparator: &impl Fn(&i32, &i32) -> i32,
) -> (u32, u32) {
    let mut left: Vec<i32> = data[start..=mid].to_vec();
    let mut right: Vec<i32> = data[(mid + 1)..=end].to_vec();
    // wheh sorting in ascending order must be MAX, otherwise MIN
    left.push(std::i32::MIN);
    right.push(std::i32::MIN);

    let mut comparisons = 0u32;
    let mut swaps = 0u32;

    let mut i = 0;
    let mut j = 0;
    for index in start..=end {
        if comparator(&left[i], &right[j]) < 0 {
            data[index] = left[i];
            i += 1;
            comparisons += 1;
        } else {
            data[index] = right[j];
            j += 1;
            comparisons += 1;
            swaps += 1;
        }
    }
    (comparisons, swaps)
}
