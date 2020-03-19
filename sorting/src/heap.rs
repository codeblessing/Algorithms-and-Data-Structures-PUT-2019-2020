use crate::errors;
use std::cmp::Ordering;

#[allow(dead_code)]
pub enum HeapType {
    MAX,
    MIN,
}

pub struct Heap {
    heap: Vec<u32>,
    comparator: fn(u32, u32) -> i32,
    comparisons: u32,
    swaps: u32,
}

impl Heap {
    fn min_comparator(first: u32, second: u32) -> i32 {
        second as i32 - first as i32
    }

    fn max_comparator(first: u32, second: u32) -> i32 {
        first as i32 - second as i32
    }

    /// Creates new heap of given type from given data.
    /// Do not change given array only borrow it and copy data.
    pub fn new(data: &[u32], heap_type: HeapType) -> Heap {
        let comparator: fn(u32, u32) -> i32 = match heap_type {
            HeapType::MIN => Heap::min_comparator,
            HeapType::MAX => Heap::max_comparator,
        };
        let mut heap = Heap {
            heap: data.to_vec(),
            comparator,
            comparisons: 0,
            swaps: 0,
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

        if left < self.heap.len() && (self.comparator)(self.heap[left], self.heap[index]) > 0 {
            largest = left;
        } else {
            largest = index;
        };
        if right < self.heap.len() && (self.comparator)(self.heap[right], self.heap[largest]) > 0 {
            largest = right;
        };
        self.comparisons += 2;

        if largest != index {
            self.heap.swap(index, largest);
            self.swaps += 1;
            self.heapify(largest);
        };
    }

    /// Removes and returns root element of the heap.
    /// # Errors
    /// If heap is empty returns `EmptyHeap` error.
    pub fn root(&mut self) -> Result<u32, errors::EmptyHeap> {
        match self.size().cmp(&1) {
            Ordering::Less => Err(errors::EmptyHeap {}),
            Ordering::Equal => Ok(self.heap.remove(1)),
            Ordering::Greater => {
                let root = self.heap[1];
                self.heap[1] = self.heap.remove(self.size());
                self.heapify(1);
                Ok(root)
            }
        }
    }

    /// Returns heap size
    pub fn size(&self) -> usize {
        self.heap.len() - 1
    }

    /// Builds heap using bottom-up heapify.
    fn build(&mut self) {
        let index = (self.size() >> 1) + 1;
        for idx in (1..index).rev() {
            self.heapify(idx);
        }
    }
}

/// Creates heap from given data and sorts it using heapyfying.
/// `order` - defines what type of heap is created.
/// HeapType::MIN sorts in ascending order, HeapType::MAX sorts in descending order.
pub fn sort(data: &[u32], order: HeapType) -> (Vec<u32>, u32, u32) {
    let mut sorted: Vec<u32> = Vec::with_capacity(data.len());
    let mut heap = Heap::new(data, order);
    heap.comparisons = 0;
    heap.swaps = 0;

    for _ in 1..=heap.size() {
        sorted.push(heap.root().unwrap());
    }

    (sorted, heap.comparisons, heap.swaps)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_heapsort_antiordered() {
        let unordered: Vec<u32> = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        let ordered = sort(&unordered, HeapType::MAX);
        assert_eq!(ordered.0, vec![19, 17, 15, 13, 11, 9, 7, 5, 3, 1]);
    }

    #[test]
    fn test_heapsort_ordered() {
        let unordered: Vec<u32> = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let ordered = sort(&unordered, HeapType::MAX);
        assert_eq!(ordered.0, unordered);
    }

    #[test]
    fn test_heapsort_random() {
        let unordered: Vec<u32> = vec![1, 15, 4, 1905, 45, 14, 5, 15, 2, 100];
        let sorted = sort(&unordered, HeapType::MAX);
        assert_eq!(sorted.0, vec![1905, 100, 45, 15, 15, 14, 5, 4, 2, 1])
    }

    #[test]
    fn test_heapsort_a_shape() {
        let unordered: Vec<u32> = vec![3, 7, 11, 15, 19, 17, 13, 9, 5, 1];
        let sorted = sort(&unordered, HeapType::MAX);
        assert_eq!(sorted.0, vec![19, 17, 15, 13, 11, 9, 7, 5, 3, 1]);
    }

    #[test]
    fn test_heapsort_v_shape() {
        let unordered: Vec<u32> = vec![17, 13, 9, 5, 1, 3, 7, 11, 15, 19];
        let sorted = sort(&unordered, HeapType::MAX);
        assert_eq!(sorted.0, vec![19, 17, 15, 13, 11, 9, 7, 5, 3, 1]);
    }
}
