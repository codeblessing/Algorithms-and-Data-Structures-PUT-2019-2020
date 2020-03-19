pub fn sort(data: &[u32], comparator: impl Fn(u32, u32) -> i32) -> (Vec<u32>, u32, u32, Vec<u32>) {
    let mut sorted = data.to_vec();

    let mut pivots: Vec<u32> = vec![];
    let mut comparisons = 0u32;
    let mut swaps = 0u32;

    let len = sorted.len();
    _quicksort_(
        &mut sorted,
        0isize,
        (len - 1) as isize,
        &comparator,
        &mut pivots,
        &mut comparisons,
        &mut swaps,
    );

    (sorted, comparisons, swaps, pivots)
}

// 3-way partitioning quicksort. Slightly improved Hoare's algorithm based on
// https://www.cs.princeton.edu/~rs/talks/QuicksortIsOptimal.pdf
fn _quicksort_(
    array: &mut [u32],
    left: isize,
    right: isize,
    compare: &impl Fn(u32, u32) -> i32,
    pivots: &mut Vec<u32>,
    comparisons: &mut u32,
    swaps: &mut u32,
) {
    if right <= left {
        return;
    }

    let mut inorder: isize = left - 1;
    let mut revorder: isize = right;
    let mut p: isize = inorder;
    let mut q: isize = revorder;
    unsafe {
        let pivot: *mut u32 = &mut array[right as usize];
        pivots.push(*pivot);
        loop {
            inorder += 1;
            while {
                *comparisons += 1;
                compare(array[inorder as usize], *pivot) < 0
            } {
                inorder += 1
            }
            revorder -= 1;
            while {
                *comparisons += 1;
                compare(*pivot, array[revorder as usize]) < 0
            } {
                if revorder == left {
                    break;
                }
                revorder -= 1;
            }
            if inorder >= revorder {
                break;
            }
            *swaps += 1;
            array.swap(inorder as usize, revorder as usize);
            if compare(array[inorder as usize], *pivot) == 0 {
                p += 1;
                array.swap(p as usize, inorder as usize)
            }
            if compare(*pivot, array[revorder as usize]) == 0 {
                q -= 1;
                array.swap(revorder as usize, q as usize)
            }
        }

        *swaps += 1;
        array.swap(inorder as usize, right as usize);
        revorder = inorder - 1;
        inorder += 1;
        let mut k: isize = left;
        while k < p {
            *swaps += 1;
            array.swap(k as usize, revorder as usize);
            k += 1;
            revorder -= 1;
            assert!(k < array.len() as isize);
        }
        k = right - 1;
        while k > q {
            *swaps += 1;
            array.swap(inorder as usize, k as usize);
            k -= 1;
            inorder += 1;
            assert!(k != 0);
        }
    }
    _quicksort_(array, left, revorder, compare, pivots, comparisons, swaps);
    _quicksort_(array, inorder, right, compare, pivots, comparisons, swaps);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_quicksort_ordered() {
        let unordered: Vec<u32> = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let ordered = sort(&unordered, |x, y| y as i32 - x as i32);
        assert_eq!(ordered.0, unordered);
    }
    #[test]
    fn test_quicksort_antiordered() {
        let unordered: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let ordered = sort(&unordered, |x, y| y as i32 - x as i32);
        assert_eq!(ordered.0, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_quicksort_random() {
        let unordered: Vec<u32> = vec![15, 4, 8, 5, 16, 1, 21, 3, 30, 6];
        let ordered = sort(&unordered, |x, y| y as i32 - x as i32);
        assert_eq!(ordered.0, vec![30, 21, 16, 15, 8, 6, 5, 4, 3, 1]);
    }
    #[test]
    fn test_quicksort_a_shape() {
        let unordered: Vec<u32> = vec![2, 4, 6, 8, 10, 9, 7, 5, 3, 1];
        let ordered = sort(&unordered, |x, y| y as i32 - x as i32);
        assert_eq!(ordered.0, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }
    #[test]
    fn test_quicksort_v_shape() {
        let unordered: Vec<u32> = vec![9, 7, 5, 3, 1, 2, 4, 6, 8, 10];
        let ordered = sort(&unordered, |x, y| y as i32 - x as i32);
        assert_eq!(ordered.0, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }
}
