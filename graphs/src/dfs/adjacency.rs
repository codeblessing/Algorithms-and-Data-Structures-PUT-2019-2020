//! Topological sorting using DFS (Depth First Search) techniqe and adjacency matrix graph representation.
#![allow(dead_code)]
use crate::graph::adjacency;
use std::iter;

#[cfg(debug_assertions)]
fn pretty_matrix(matrix: &[Vec<i8>]) -> String {
    let mut out: String = String::new();
    for line in matrix {
        out += line
            .iter()
            .map(|val| format!("{:^3}", val))
            .fold(String::new(), |acc, val| format!("{}{}", acc, val))
            .chars()
            .chain(iter::once('\n'))
            .collect::<Vec<char>>()
            .iter()
            .cloned()
            .collect::<String>()
            .as_str();
    }
    out
}

pub fn sort(matrix: &adjacency::Matrix) -> Vec<usize> {
    let mut matrix = matrix.matrix();
    let mut vertices: Vec<usize> = (1..matrix.len()).collect();
    if vertices.is_empty() {
        return Vec::new();
    }

    let mut stack: Vec<usize> = Vec::new();
    let mut sorted: Vec<usize> = Vec::new();

    stack.push(vertices.remove(0));
    let mut vertex: usize;
    loop {
        // Get last vertex from stack and color it grey.
        eprintln!("Stack {:?}", stack);
        vertex = stack[stack.len() - 1];
        matrix.iter_mut().for_each(|val| {
            val[vertex] = match val[vertex] {
                -1 => -2,
                1 => 2,
                val => val,
            }
        });

        let next = matrix[vertex]
            .iter()
            .enumerate()
            .find(|&(_, &val)| val == 1);
        let next = match next {
            None => None,
            Some((key, _)) => Some(key),
        };

        #[cfg(debug_assertions)]
        eprintln!("Vertex: {}\n{}\n{:?}", vertex, pretty_matrix(&matrix), next);

        // If vertex have grey successor there's cycle and we can't sort this graph.
        if let Some(_) = matrix[vertex].iter().find(|&&val| val == 2) {
            println!("Cannot sort graph with cycles.");
            sorted = vec![];
            break;
        }
        // If vertex have successors.
        if let Some(val) = next {
            stack.push(vertices.remove(index_of(val, &vertices).unwrap()));
        }
        // If vertex have no successors.
        else {
            // Color it black.
            sorted.push(stack.pop().unwrap());
            matrix.iter_mut().for_each(|line| line[vertex] *= 2);
            // If there's no more visited (stack) and unvisited (white) vertices - end.
            if stack.is_empty() && vertices.is_empty() {
                break;
            }
            // If there's no more visited (stack) vertices but there are vertices not visited (white) yet.
            if stack.is_empty() {
                stack.push(vertices.remove(0));
            }
        }
    }

    sorted.iter().rev().copied().collect()
}

fn index_of(value: usize, vec: &[usize]) -> Option<usize> {
    let index = vec.iter().enumerate().find(|(_, &val)| val == value);
    match index {
        Some((key, _)) => Some(key),
        None => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::graph::adjacency;
    #[test]
    fn test_sort_neighbourhood() {
        let graph: Vec<(usize, usize)> = vec![(1, 2), (1, 5), (1, 3), (2, 5), (3, 4), (4, 5)];
        let neigh_matrix = adjacency::Matrix::from(&graph);
        let sorted: Vec<usize> = sort(&neigh_matrix);
        assert_eq!(sorted, vec![1, 3, 4, 2, 5]);
    }

    #[test]
    fn test_sort_with_cycle() {
        let graph: Vec<(usize, usize)> =
            vec![(1, 5), (1, 3), (2, 1), (2, 5), (3, 2), (3, 4), (4, 5)];
        let neigh_matrix = adjacency::Matrix::from(&graph);
        let sorted = sort(&neigh_matrix);
        assert_eq!(sorted, vec![])
    }
}
