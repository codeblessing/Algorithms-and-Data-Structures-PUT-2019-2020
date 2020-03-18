use super::errors;

static mut COMPARISONS: u32 = 0u32;
static mut SWAPS: u32 = 0u32;

#[allow(dead_code)]
pub enum HeapType {
    MAX,
    MIN,
}

pub struct Heap {
    heap: Vec<i32>,
    comparator: fn(&i32, &i32) -> i32,
}

impl Heap {
    fn min_comparator(first: &i32, second: &i32) -> i32 {
        second - first
    }

    fn max_comparator(first: &i32, second: &i32) -> i32 {
        first - second
    }

    /// Creates new heap of given type from given data.
    /// Do not change given array only borrow it and copy data.
    pub fn new(data: &Vec<i32>, heap_type: HeapType) -> Heap {
        let comparator: fn(&i32, &i32) -> i32 = match heap_type {
            HeapType::MIN => Heap::min_comparator,
            HeapType::MAX => Heap::max_comparator,
        };
        let mut heap = Heap {
            heap: data.clone(),
            comparator,
        };

        heap.heap.insert(0, 0);
        heap.build();
        heap
    }

    /// Returns index of given node left child
    pub fn left(&self, index: usize) -> usize {
        index << 1
    }

    /// Returns index of given node right child
    pub fn right(&self, index: usize) -> usize {
        (index << 1) + 1
    }

    /// Restore heap property in given node and its children.
    fn heapify(&mut self, index: usize) {
        let left = self.left(index);
        let right = self.right(index);
        let mut largest: usize;

        unsafe {
            if left < self.heap.len() && (self.comparator)(&self.heap[left], &self.heap[index]) > 0
            {
                largest = left;
                SWAPS += 1;
            } else {
                largest = index;
                SWAPS += 1;
            };
            COMPARISONS += 1;

            if right < self.heap.len()
                && (self.comparator)(&self.heap[right], &self.heap[largest]) > 0
            {
                largest = right;
                SWAPS += 1;
            };
            COMPARISONS += 1;
        }
        if largest != index {
            self.heap.swap(index, largest);
            self.heapify(largest);
        };
    }

    /// Removes and returns root element of the heap.
    /// # Errors
    /// If heap is empty returns `EmptyHeap` error.
    pub fn root(&mut self) -> Result<i32, errors::EmptyHeap> {
        if self.size() < 1 {
            Err(errors::EmptyHeap {})
        } else if self.size() == 1 {
            Ok(self.heap.remove(1))
        } else {
            let root = self.heap[1];
            self.heap[1] = self.heap.remove(self.heap.len() - 1);
            self.heapify(1);
            Ok(root)
        }
    }

    /// Returns heap size
    pub fn size(&self) -> usize {
        self.heap.len() - 1
    }

    /// Builds heap using bottom-up heapify.
    fn build(&mut self) {
        let index = (self.heap.len() / 2) + 1;
        for idx in (1..index).rev() {
            self.heapify(idx);
            unsafe {
                SWAPS = 0u32;
                COMPARISONS = 0u32;
            }
        }
    }
}

pub fn sort(data: &Vec<i32>, order: HeapType) -> (Vec<i32>, u32, u32) {
    let mut sorted: Vec<i32> = vec![];
    let mut heap = Heap::new(data, order);

    let mut comparisons: u32 = 0u32;
    let mut swaps: u32 = 0u32;

    for _ in 1..=heap.size() {
        sorted.push(heap.root().unwrap());
        unsafe {
            comparisons += COMPARISONS;
            swaps += SWAPS;
            COMPARISONS = 0;
            SWAPS = 0;
        }
    }

    (sorted, comparisons, swaps)
}
