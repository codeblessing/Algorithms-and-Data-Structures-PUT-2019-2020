pub fn sort(data: &Vec<i32>, comparator: impl Fn(&i32, &i32) -> i32) -> Vec<i32> {
    let mut out = data.clone();
    let length = out.len() - 1;
    _sort_(&mut out, 0, length, &comparator);
    return out;
}

fn _sort_(data: &mut Vec<i32>, start: usize, end: usize, comparator: &impl Fn(&i32, &i32) -> i32) {
    if start < end {
        let mid = (start + end) / 2;
        _sort_(data, start, mid, &comparator);
        _sort_(data, mid + 1, end, &comparator);
        _merge_(data, start, mid, end, &comparator);
    }
}

fn _merge_(
    data: &mut Vec<i32>,
    start: usize,
    mid: usize,
    end: usize,
    comparator: &impl Fn(&i32, &i32) -> i32,
) {
    let left: Vec<i32> = data[start..=mid].to_vec();
    let right: Vec<i32> = data[(mid + 1)..=end].to_vec();

    let mut i = start;
    let mut j = mid;
    for index in start..=end {
        if i >= mid {
            (**data)[index] = right[j];
            j += 1;
        } else if j >= end {
            (**data)[index] = left[i];
            i += 1;
        } else if comparator(&left[i], &right[j]) < 0 {
            (**data)[index] = left[i];
            i += 1;
        } else {
            (**data)[index] += right[j];
            j += 1;
        }
    }
}
