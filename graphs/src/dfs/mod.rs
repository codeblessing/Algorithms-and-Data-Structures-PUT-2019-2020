pub mod adjacency;
pub mod graph;
pub mod successors_list;

struct ColoredVertex {
    value: usize,
    color: Color,
}

#[derive(PartialEq)]
enum Color {
    White,
    Grey,
    Black,
}

impl PartialEq for ColoredVertex {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for ColoredVertex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

impl From<usize> for ColoredVertex {
    fn from(val: usize) -> Self {
        Self {
            value: val,
            color: Color::White,
        }
    }
}

impl ColoredVertex {
    pub fn color(&mut self, color: Color) {
        self.color = color;
    }
}
