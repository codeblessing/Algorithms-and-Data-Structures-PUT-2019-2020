//! Module for representing graph as adjacency matrix.
use std::collections::HashSet;

/// Represents graph as adjacency matrix.
pub struct Matrix {
    matrix: Vec<Vec<i8>>,
}

impl Matrix {
    /// Creates adjacency matrix from vertex arcs list. Removes redundant arcs.
    pub fn from(data: &[(usize, usize)]) -> Self {
        let vertices: Vec<usize> = data
            .iter()
            .fold(HashSet::new(), |mut acc, val| {
                acc.insert(val.0);
                acc.insert(val.1);
                acc
            })
            .iter()
            .copied()
            .collect();

        if vertices.is_empty() {
            return Matrix { matrix: vec![] };
        }

        let max = vertices.iter().max().unwrap();
            
        let mut matrix: Vec<Vec<i8>> = vec![vec![0; max + 1]; max + 1];

        for &(a, b) in data {
            matrix[a][b] = 1;
            if matrix[b][a] == 0 {
                matrix[b][a] = -1;
            }
        }

        Matrix { matrix }
    }

    /// Returns copy of adjacency matrix as Vec<Vec<i8>>
    pub fn matrix(&self) -> Vec<Vec<i8>> {
        self.matrix.clone()
    }
}

#[allow(dead_code)]
struct SizeError {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_matrix() {
        let test_data: Vec<(usize, usize)> = vec![
            (1, 2),
            (1, 5),
            (2, 1),
            (2, 3),
            (2, 4),
            (3, 2),
            (3, 5),
            (4, 2),
            (4, 5),
            (5, 1),
        ];
        let built_matrix = Matrix::from(&test_data).matrix();
        let matrix: Vec<Vec<i8>> = vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 1],
            vec![0, 1, 0, 1, 1, 0],
            vec![0, 0, 1, 0, 0, 1],
            vec![0, 0, 1, 0, 0, 1],
            vec![0, 1, 0, -1, -1, 0],
        ];

        assert_eq!(built_matrix, matrix);
    }
}
