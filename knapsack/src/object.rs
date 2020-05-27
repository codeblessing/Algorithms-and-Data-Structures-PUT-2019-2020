#[derive(Clone, Copy)]
pub struct Knapsack {
    pub capacity: usize
}

#[derive(Copy, Clone)]
pub struct Object {
    pub weight: usize,
    pub value: usize
}