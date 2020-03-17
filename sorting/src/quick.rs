static mut COMPARISONS: u32 = 0;
static mut SWAPS: u32 = 0;

pub fn sort(data: &Vec<i32>, comparator: impl Fn(&i32, &i32) -> i32) -> (Vec<i32>, u32, u32) {
    let mut sorted = data.clone();
    let len = sorted.len();
    _quicksort_(&mut sorted, 0isize, (len - 1) as isize, &comparator);
    unsafe {
        let out = (sorted, COMPARISONS, SWAPS);
        COMPARISONS = 0;
        SWAPS = 0;
        return out;
    }
}

// 3-way partitioning quicksort. Slightly improved Hoare's algorithm based on
// https://www.cs.princeton.edu/~rs/talks/QuicksortIsOptimal.pdf
fn _quicksort_(array: &mut [i32], left: isize, right: isize, compare: &impl Fn(&i32, &i32) -> i32) {
    if right <= left {
        return;
    }

    let mut i: isize = left - 1;
    let mut j: isize = right;
    let mut p: isize = i;
    let mut q: isize = j;
    unsafe {
        let v: *mut i32 = &mut array[right as usize];
        loop {
            i += 1;
            while {
                COMPARISONS += 1;
                compare(&array[i as usize], &*v) < 0
            } {
                i += 1
            }
            j -= 1;
            while {
                COMPARISONS += 1;
                compare(&*v, &array[j as usize]) < 0
            } {
                if j == left {
                    break;
                }
                j -= 1;
            }
            if i >= j {
                break;
            }
            SWAPS += 1;
            array.swap(i as usize, j as usize);
            if compare(&array[i as usize], &*v) == 0 {
                p += 1;
                array.swap(p as usize, i as usize)
            }
            if compare(&*v, &array[j as usize]) == 0 {
                q -= 1;
                array.swap(j as usize, q as usize)
            }
        }

        SWAPS += 1;
        array.swap(i as usize, right as usize);
        j = i - 1;
        i += 1;
        let mut k: isize = left;
        while k < p {
            SWAPS += 1;
            array.swap(k as usize, j as usize);
            k += 1;
            j -= 1;
            assert!(k < array.len() as isize);
        }
        k = right - 1;
        while k > q {
            SWAPS += 1;
            array.swap(i as usize, k as usize);
            k -= 1;
            i += 1;
            assert!(k != 0);
        }
    }
    _quicksort_(array, left, j, compare);
    _quicksort_(array, i, right, compare);
}
