use crate::insertion;

pub fn sort(
    data: &Vec<u32>,
    comparator: impl Fn(u32, u32) -> i32,
) -> (Vec<u32>, u32, u32, Vec<u32>) {
    let mut knuth_index = ((((data.len() << 1) / 3) + 1) as f32).log(3f32) as u32;
    let mut knuth_delta: u32 = (3u32.pow(knuth_index) - 1) >> 1;
    let mut sorted: Vec<u32> = vec![];
    let mut comparisons: u32 = 0;
    let mut swaps = 0u32;
    let mut deltas: Vec<u32> = vec![];

    while knuth_delta > 0 {
        let result = shellsort(data, knuth_delta, &comparator);
        sorted = result.0;
        comparisons += result.1;
        swaps += result.2;
        deltas.push(knuth_delta);

        knuth_index -= 1;
        knuth_delta = (3u32.pow(knuth_index) - 1) >> 1;
    }

    return (sorted, comparisons, swaps, deltas);
}

fn shellsort(
    data: &Vec<u32>,
    delta: u32,
    comparator: impl Fn(u32, u32) -> i32,
) -> (Vec<u32>, u32, u32) {
    let mut sorted: Vec<u32> = vec![0; data.len()];
    let mut chunk: Vec<u32>;

    let mut comparisons = 0u32;
    let mut swaps = 0u32;

    for index in 0..delta {
        chunk = vec![];
        for item in data.iter().skip(index as usize).step_by(delta as usize) {
            chunk.push(*item);
        }

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
