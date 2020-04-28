//! Topological sorting using DFS (Depth First Search) techniqe and graph matrix graph representation.
#![allow(dead_code)]
use crate::graph;
use linked_hash_set::LinkedHashSet;

pub fn sort(matrix: &graph::Matrix) -> Vec<usize> {
    if matrix.vertex_count() == 0 {
        return vec![];
    }

    let size = matrix.vertex_count();
    let mut vertices = matrix.vertices();
    let mut matrix = matrix.matrix();
    eprintln!("{:?}", vertices);

    let s = size + 1;

    let mut stack: LinkedHashSet<usize> = LinkedHashSet::new();
    let mut sorted: Vec<usize> = Vec::new();

    let mut successor: usize;
    let mut vertex: usize;

    stack.insert(vertices.remove(0));

    loop {
        vertex = *stack.back().unwrap();

        successor = matrix[vertex][s] as usize;
        let next_successor = if matrix[vertex][successor] == successor as isize {
            0
        } else {
            matrix[vertex][successor]
        };

        eprintln!("Successor: {}", successor);
        eprintln!("Next successor: {}", next_successor);

        matrix[vertex][s] = next_successor as isize;

        eprintln!("Verts: {:?}", vertices);
        eprintln!("Stack: {:?}", stack);
        eprintln!("Sorted: {:?}", sorted);

        eprintln!("{}", pretty_matrix(&matrix));

        // If vertex have no successors
        if successor == 0 {
            // Color it black and remove occurrences in matrix.
            sorted.push(stack.pop_back().unwrap());
            matrix.iter_mut().for_each(|row| {
                let succ = if row[vertex] == vertex as isize { 0 } else { row[vertex] };
                for (key, &val) in row.clone().iter().enumerate() {
                    if val == vertex as isize {
                        row[key] = succ;
                    }
                }

            });
            // If stack is empty and there's no more vertices unvisited - end.
            if stack.is_empty() && vertices.is_empty() {
                break;
            }
            // If stack is empty but there are unvisited vertices.
            if stack.is_empty() {
                stack.insert(vertices.remove(0));
            }
        }
        // If vertex have successor on stack there's a cycle and we can't sort this graph.
        else if !stack.insert_if_absent(successor) {
            println!("Cannot sort graph with cycles.");
            sorted = vec![];
            break;
        } else {
            vertices = vertices
                .iter()
                .filter(|&&val| val != successor)
                .copied()
                .collect();
        }
    }

    sorted.iter().rev().copied().collect()
}

fn pretty_matrix(matrix: &[Vec<isize>]) -> String {
    let mut out: String = String::new();
    for line in matrix {
        out += line
            .iter()
            .map(|val| format!("{:^3}", val))
            .fold(String::new(), |acc, val| format!("{}{}", acc, val))
            .chars()
            .chain(std::iter::once('\n'))
            .collect::<Vec<char>>()
            .iter()
            .cloned()
            .collect::<String>()
            .as_str();
    }
    out
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sort_graph_matrix() {
        let graph: Vec<(usize, usize)> = vec![(1, 2), (1, 5), (1, 3), (2, 5), (3, 4), (4, 5)];
        let matrix = graph::Matrix::from(&graph, 5);
        let sorted = sort(&matrix);
        assert_eq!(sorted, vec![1, 3, 4, 2, 5])
    }

    #[test]
    fn test_sort_with_cycle() {
        let graph: Vec<(usize, usize)> =
            vec![(1, 5), (1, 3), (2, 1), (2, 5), (3, 2), (3, 4), (4, 5)];
        let graph_matrix = graph::Matrix::from(&graph, 5);
        let sorted = sort(&graph_matrix);
        assert_eq!(sorted, vec![])
    }
}
