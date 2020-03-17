use crate::insertion;

pub fn sort(
    data: &Vec<i32>,
    knuth_index: &mut u32,
    comparator: impl Fn(&i32, &i32) -> i32,
) -> (Vec<i32>, u32, u32) {
    let mut knuth_delta: u32 = ((3i32.pow(*knuth_index) - 1) / 2) as u32;
    let mut sorted: Vec<i32> = vec![];
    let mut comparisons: u32 = 0;
    let mut swaps = 0u32;

    while knuth_delta > 0 {
        let (srtd, comps, swps) = shellsort(data, knuth_delta, &comparator);
        sorted = srtd;
        comparisons += comps;
        swaps += swps;

        *knuth_index -= 1;
        knuth_delta = ((3i32.pow(*knuth_index) - 1) / 2) as u32;
    }

    return (sorted, comparisons, swaps);
}

pub fn classic_shellsort(
    data: &Vec<i32>,
    comparator: impl Fn(&i32, &i32) -> i32,
) -> (Vec<i32>, u32, u32) {
    let mut delta = (data.len() / 2) as u32;
    let mut sorted: Vec<i32> = vec![];
    let mut comparisons: u32 = 0;
    let mut swaps = 0u32;

    while delta > 0 {
        let (srtd, comps, swps) = shellsort(&data, delta, &comparator);
        sorted = srtd;
        comparisons += comps;
        swaps += swps;
        delta = (delta / 2) as u32;
    }

    return (sorted, comparisons, swaps);
}

fn shellsort(
    data: &Vec<i32>,
    delta: u32,
    comparator: impl Fn(&i32, &i32) -> i32,
) -> (Vec<i32>, u32, u32) {
    let mut sorted: Vec<i32> = vec![0; data.len()];
    let mut chunk: Vec<i32>;
    let mut comparisons = 0u32;
    let mut swaps = 0u32;

    for index in 0..delta {
        // High time cost ~10'000ns for 10 element Vec
        chunk = data
            .iter()
            .enumerate()
            .filter(|&item| (item.0 as u32) % delta == index)
            .map(|item| *item.1)
            .collect();

        let (sorted_chunk, comps, swps) = insertion::sort(&chunk, &comparator);
        comparisons = comps;
        swaps = swps;

        for (idx, &item) in sorted_chunk.iter().enumerate() {
            let insert_index = index as usize + (idx * delta as usize);
            sorted[insert_index] = item;
        }
    }
    return (sorted, comparisons, swaps);
}
