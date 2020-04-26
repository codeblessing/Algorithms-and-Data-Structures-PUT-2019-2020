//! Topological sorting using zero-in node removal technique and adjascency matrix graph representation.
use crate::graph::adjacency;
use std::collections::HashMap;

/// Sorts graph using zero-in node removal technique and adjacency matrix graph representation.
/// When graph contains cycle returns empty `Vec` otherwise returns `Vec` with sorted nodes.
pub fn sort(matrix: &adjacency::Matrix) -> Vec<usize> {
    let matrix = matrix.matrix();
    let vertices: Vec<usize> = (1..matrix.len()).collect();

    let mut predecessors: HashMap<usize, Vec<usize>> = HashMap::new();
    vertices.iter().for_each(|&key| {
        let mut preds: Vec<usize> = Vec::new();
        for i in 1..matrix.len() {
            if matrix[i][key] == 1 {
                preds.push(i);
            }
        }
        predecessors.insert(key, preds);
    });

    eprintln!("{:?}", predecessors);

    let mut sorted: Vec<usize> = Vec::new();
    let mut vertex: usize;

    loop {
        match predecessors.iter().find(|(_, preds)| preds.is_empty()) {
            None => {
                eprintln!("Cannot sort graph with cycles.");
                sorted = vec![];
                break;
            }
            Some((&key, _)) => vertex = key,
        }

        sorted.push(vertex);

        predecessors.iter_mut().for_each(|(_, preds)| {
            *preds = preds
                .iter()
                .filter(|&&val| val != vertex)
                .copied()
                .collect();
        });

        predecessors.remove(&vertex);

        eprintln!("Predecessors: {:?}", predecessors);
        eprintln!("Sorted: {:?}", sorted);

        if predecessors.is_empty() {
            break;
        }
    }

    sorted
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::graph::adjacency;

    #[test]
    fn test_sort_graph_adjacency_matrix() {
        let graph: Vec<(usize, usize)> = vec![(1, 2), (1, 5), (1, 3), (2, 5), (3, 2), (3, 4), (4, 2), (4, 5)];
        let matrix = adjacency::Matrix::from(&graph);
        let sorted = sort(&matrix);
        assert_eq!(sorted, vec![1, 3, 4, 2, 5])
    }

    #[test]
    fn test_sort_with_cycle() {
        let graph: Vec<(usize, usize)> =
            vec![(1, 5), (1, 3), (2, 1), (2, 5), (3, 2), (3, 4), (4, 5)];
        let matrix = adjacency::Matrix::from(&graph);
        let sorted = sort(&matrix);
        assert_eq!(sorted, vec![])
    }
}