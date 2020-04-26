//! Topological sorting using zero-in node removal technique and successors list graph representation.

use crate::graph::successors;
use std::collections::HashMap;

/// Sorts graph using zero-in node removal technique and successors list graph representation.
/// When graph contains cycle returns empty `Vec` otherwise returns `Vec` with sorted nodes.
pub fn sort(list: &successors::List) -> Vec<usize> {
    let list = list.list();
    let vertices: Vec<usize> = list.iter().map(|(&key, _)| key).collect();

    if vertices.is_empty() {
        return vec![];
    }

    let mut predecessors: HashMap<usize, Vec<usize>> = HashMap::new();
    vertices.iter().for_each(|&key| {
        let mut preds: Vec<usize> = Vec::new();
        list.iter().for_each(|(&pred, values)| {
            if values.iter().find(|&&val| val == key).is_some() {
                preds.push(pred);
            }
        });
        predecessors.insert(key, preds);
    });

    let mut sorted: Vec<usize> = Vec::new();
    let mut vertex: usize;

    loop {
        match predecessors.iter().find(|(_, preds)| preds.is_empty()) {
            Some((&key, _)) => vertex = key,
            None => {
                eprintln!("Cannot sort graph with cycles.");
                sorted = vec![];
                break;
            }
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

        if predecessors.is_empty() {
            break;
        }
    }

    sorted
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::graph::successors;

    #[test]
    fn test_sort_graph_successor_list() {
        let graph: Vec<(usize, usize)> = vec![(1, 2), (1, 5), (1, 3), (2, 5), (3, 2), (3, 4), (4, 2), (4, 5)];
        let list = successors::List::from(&graph);
        let sorted = sort(&list);
        assert_eq!(sorted, vec![1, 3, 4, 2, 5])
    }

    #[test]
    fn test_sort_with_cycle() {
        let graph: Vec<(usize, usize)> =
            vec![(1, 5), (1, 3), (2, 1), (2, 5), (3, 2), (3, 4), (4, 5)];
        let graph_matrix = successors::List::from(&graph);
        let sorted = sort(&graph_matrix);
        assert_eq!(sorted, vec![])
    }
}