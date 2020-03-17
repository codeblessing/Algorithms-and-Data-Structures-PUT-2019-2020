use super::errors;
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

    /// Creates new heap of given type from given data. Do not change given array only borrow it and copy data.
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
        if left < self.heap.len() && (self.comparator)(&self.heap[left], &self.heap[index]) > 0 {
            largest = left;
        } else {
            largest = index;
        };
        if right < self.heap.len() && (self.comparator)(&self.heap[right], &self.heap[largest]) > 0
        {
            largest = right;
        };
        if largest != index {
            let tmp = self.heap[index];
            self.heap[index] = self.heap[largest];
            self.heap[largest] = tmp;
            self.heapify(largest);
        };
    }

    /// Removes and returns root element of the heap.
    /// # Errors
    /// If heap is empty returns `EmptyHeap` error.
    pub fn root(&mut self) -> Result<i32, errors::EmptyHeap> {
        if self.heap.len() < 2 {
            Err(errors::EmptyHeap {})
        } else if self.heap.len() == 2 {
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
        self.heap.len()
    }

    /// Builds heap using bottom-up heapify.
    fn build(&mut self) {
        let index = (self.heap.len() / 2) + 1;
        for idx in (1..index).rev() {
            self.heapify(idx);
        }
    }
}

pub fn sort(data: &Vec<i32>, order: HeapType) -> Vec<i32> {
    let mut sorted: Vec<i32> = vec![];
    let mut heap = Heap::new(data, order);
    for _ in 1..heap.size() {
        sorted.push(heap.root().unwrap());
    }
    sorted
}
