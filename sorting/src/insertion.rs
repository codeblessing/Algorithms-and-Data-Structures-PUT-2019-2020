/// Sorts data using Insertion Sort algorithm.
/// * `comparator`: ordering function or clojure
/// * `data`: immutable borrow of unordered array.
pub fn sort(data: &Vec<i32>, comparator: impl Fn(&i32, &i32) -> i32) -> (Vec<i32>, u32, u32) {
    // Only to fulfill testing requirements.
    // http://www.cs.put.poznan.pl/mszachniuk/mszachniuk_files/lab_aisd/aisdzadania.html:
    // Dane wyjściowe:
    // - liczba operacji porównania i liczba operacji zamiany elementów podczas sortowania.
    let mut comparisons = 0u32;
    let mut swaps = 0u32;

    let mut sorted = vec![data[0]];

    // Checks every element with previously inserted.
    for item in &data[1..] {
        let mut index = sorted.len();
        // Iterates through `sorted` in reverse order (from end to start).
        for compared in sorted.iter().rev() {
            let compare = comparator(compared, item);
            comparisons += 1;

            // If element is less than or equal to compared item it should go right before it.
            // Warning! This move makes this implementation unstable. To "stabilize" differenciate "less" ordering
            // from "equal" ordering.
            // `insert` inserts given element at given index and shifts all elements with equal/higher index to right.
            if compare <= 0 {
                sorted.insert(index as usize, *item);
                break;
            }
            // If index == 1 there's no more elements to compare with.
            // It means we should insert this element at beggining of array.
            else if index == 1 {
                sorted.insert((index - 1) as usize, *item);
                break;
            }
            // If element is "greater" and we're not at the beggining of the array we must swap
            // item we want to insert with item already in `sorted`. In this implementation we don't really swap
            // elements because it is ineffective. Rather than actually swap items we only lower insertion index
            // and increase swaps count.
            else {
                swaps += 1;
                index -= 1;
            };
        }
    }

    return (sorted, comparisons, swaps);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_insertion_sort() {
        let unordered: Vec<i32> = vec![15, 4, 8, 5, 16, 1];
        let ordered = sort(&unordered, |x, y| y - x);
        assert_eq!(ordered.0, vec![16, 15, 8, 5, 4, 1]);
    }
}
