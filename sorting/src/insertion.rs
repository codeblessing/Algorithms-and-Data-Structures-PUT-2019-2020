/// Sorts data using Insertion Sort algorithm.
/// * `comparator`: ordering function or closure
/// * `data`: immutable borrow of unordered array.
pub fn sort(data: &[u32], comparator: impl Fn(u32, u32) -> i32) -> (Vec<u32>, u32, u32) {
    // Only to fulfill testing requirements.
    // http://www.cs.put.poznan.pl/mszachniuk/mszachniuk_files/lab_aisd/aisdzadania.html:
    // Dane wyjściowe:
    // - liczba operacji porównania i liczba operacji zamiany elementów podczas sortowania.
    let mut comparisons = 0u32;
    let mut swaps = 0u32;

    let mut ordered: Vec<u32> = Vec::with_capacity(data.len());
    ordered.push(data[0]);

    // Checks every element with previously inserted.
    for item in &data[1..] {
        let mut index = ordered.len();
        // Iterates through `ordered` in reverse order (from end to start).
        for compared in ordered.iter().rev() {
            let compare = comparator(*compared, *item);
            comparisons += 1;

            // If element is less than or equal to compared item it should go right before it.
            // Warning! This move makes this implementation unstable. To "stabilize" differentiate "less" ordering
            // from "equal" ordering.
            // `insert` inserts given element at given index and shifts all elements with equal/higher index to right.
            if compare <= 0 {
                ordered.insert(index, *item);
                break;
            }
            // If index == 1 there's no more elements to compare with.
            // It means we should insert this element at beggining of array.
            else if index == 1 {
                ordered.insert((index - 1) as usize, *item);
                break;
            }
            // If element is "greater" and we're not at the beggining of the array we must swap
            // item we want to insert with item already in `ordered`. In this implementation we don't really swap
            // elements because it is ineffective. Rather than actually swap items we only lower insertion index
            // and increase swaps count.
            else {
                swaps += 1;
                index -= 1;
            };
        }
    }

    (ordered, comparisons, swaps)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insertionsort_ordered() {
        let unordered: Vec<u32> = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let ordered = sort(&unordered, |x, y| y as i32 - x as i32);
        assert_eq!(ordered.0, unordered);
    }
    #[test]
    fn test_insertionsort_antiordered() {
        let unordered: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let ordered = sort(&unordered, |x, y| y as i32 - x as i32);
        assert_eq!(ordered.0, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_insertionsort_random() {
        let unordered: Vec<u32> = vec![15, 4, 8, 5, 16, 1, 21, 3, 30, 6];
        let ordered = sort(&unordered, |x, y| y as i32 - x as i32);
        assert_eq!(ordered.0, vec![30, 21, 16, 15, 8, 6, 5, 4, 3, 1]);
    }
    #[test]
    fn test_insertionsort_a_shape() {
        let unordered: Vec<u32> = vec![2, 4, 6, 8, 10, 9, 7, 5, 3, 1];
        let ordered = sort(&unordered, |x, y| y as i32 - x as i32);
        assert_eq!(ordered.0, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }
    #[test]
    fn test_insertionsort_v_shape() {
        let unordered: Vec<u32> = vec![9, 7, 5, 3, 1, 2, 4, 6, 8, 10];
        let ordered = sort(&unordered, |x, y| y as i32 - x as i32);
        assert_eq!(ordered.0, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }
}
