#![allow(dead_code)]
#![warn(clippy::all)]
//! Module for representing graph as graph matrix.

pub mod adjacency;
pub mod successors;

use std::collections::HashMap;

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
/// Represents graph as graph matrix.
pub struct Matrix {
    matrix: Vec<Vec<isize>>,
    vertex_count: usize,
    vertices: Vec<usize>
}

impl Matrix {
    /// Creates graph matrix from vertex arcs list.
    pub fn from(data: &[(usize, usize)], vertex_count: usize) -> Self {
        if vertex_count == 0 {
            return Matrix { matrix: vec![], vertex_count: 0, vertices: vec![] };
        }

        let vertices: Vec<usize> = (1..=vertex_count).collect();

        eprintln!("Vertices: {:?}\nMax vertex: {}", vertices, vertex_count);

        let mut matrix: Vec<Vec<isize>> = vec![vec![0; vertex_count + 5]; vertex_count + 1];
        let mut successors = Matrix::successors_list(data, &vertices);
        let mut predecessors = Matrix::predecessors_list(data, &vertices);
        let mut direct_loops = Matrix::direct_loop_list(data, &vertices);
        let mut unconnected = Matrix::unconnected_list(data, &vertices);

        eprintln!("{:?}", successors);
        eprintln!("{:?}", predecessors);
        eprintln!("{:?}", direct_loops);
        eprintln!("{:?}", unconnected);

        #[allow(clippy::many_single_char_names)]
        let s = vertex_count + 1;
        let p = vertex_count + 2;
        let c = vertex_count + 3;
        let u = vertex_count + 4;
        #[warn(clippy::many_single_char_names)]
        successors.iter().for_each(|(&row, succ)| {
            matrix[row][s] = if succ.is_empty() {
                0isize
            } else {
                succ[0] as isize
            }
        });

        vertices.iter().for_each(|&row| {
            let mut succ = successors.remove(&row).unwrap();
            loop {
                let current = if succ.is_empty() {
                    break;
                } else {
                    succ.remove(0)
                };
                let next = if succ.is_empty() { current } else { succ[0] };
                matrix[row][current] = next as isize;
            }
        });
        eprintln!("Successors applied:\n{}", pretty_matrix(&matrix));

        predecessors.iter().for_each(|(&row, preds)| {
            matrix[row][p] = if preds.is_empty() {
                0isize
            } else {
                preds[0] as isize
            }
        });

        vertices.iter().for_each(|&row| {
            let mut pred = predecessors.remove(&row).unwrap();
            loop {
                let current = if pred.is_empty() {
                    break;
                } else {
                    pred.remove(0)
                };
                let next = if pred.is_empty() { current } else { pred[0] };
                matrix[row][current] = (s + next) as isize;
            }
        });
        eprintln!("Predecessors applied:\n{}", pretty_matrix(&matrix));

        direct_loops.iter().for_each(|(&row, loops)| {
            matrix[row][c] = if loops.is_empty() { 0isize } else { loops[0] as isize };
        });

        vertices.iter().for_each(|&row| {
            let mut loops = direct_loops.remove(&row).unwrap();
            loop {
                let current = if loops.is_empty() {
                    break;
                } else {
                    loops.remove(0)
                };
                let next = if loops.is_empty() { current } else { loops[0] };
                matrix[row][current] = (2 * s + next) as isize;
            }
        });
        eprintln!("Direct loops applied:\n{}", pretty_matrix(&matrix));

        unconnected.iter().for_each(|(&row, uncon)| {
            matrix[row][u] = if uncon.is_empty() { 0isize } else { uncon[0] as isize }
        });

        vertices.iter().for_each(|&row| {
            let mut uncon = unconnected.remove(&row).unwrap();
            loop {
                let current = if uncon.is_empty() {
                    break;
                } else {
                    uncon.remove(0)
                };
                let next = if uncon.is_empty() { current } else { uncon[0] };
                matrix[row][current] = -(next as isize);
            }
        });
        eprintln!("Unconnected applied:\n{}", pretty_matrix(&matrix));

        Matrix { matrix, vertex_count, vertices }
    }

    /// Returns graph matrix as Vec<Vec<isize>>.
    pub fn matrix(&self) -> Vec<Vec<isize>> {
        self.matrix.clone()
    }

    pub fn vertex_count(&self) -> usize {
        self.vertex_count
    }

    pub fn vertices(&self) -> Vec<usize> {
        self.vertices.clone()
    }

    #[inline]
    fn successors_list(data: &[(usize, usize)], vertices: &[usize]) -> HashMap<usize, Vec<usize>> {
        let mut list: HashMap<usize, Vec<usize>> = HashMap::new();

        data.iter().for_each(|&(a, b)| {
            list.entry(a)
                .and_modify(|val| val.push(b))
                .or_insert_with(|| vec![b]);
        });

        vertices.iter().for_each(|&val| {
            list.entry(val).or_insert_with(|| vec![]);
        });

        list.iter_mut().for_each(|(_, val)| val.sort_unstable());

        list
    }

    #[inline]
    fn predecessors_list(
        data: &[(usize, usize)],
        vertices: &[usize],
    ) -> HashMap<usize, Vec<usize>> {
        let mut list: HashMap<usize, Vec<usize>> = HashMap::new();

        data.iter().for_each(|&(a, b)| {
            list.entry(b)
                .and_modify(|val| val.push(a))
                .or_insert_with(|| vec![a]);
        });

        vertices.iter().for_each(|&val| {
            list.entry(val).or_insert_with(|| vec![]);
        });

        list.iter_mut().for_each(|(_, val)| val.sort_unstable());

        list
    }

    #[inline]
    fn unconnected_list(
        data: &[(usize, usize)],
        vertices: &[usize],
    ) -> HashMap<usize, Vec<usize>> {
        let &size = vertices.iter().max().unwrap();

        let mut matrix: Vec<Vec<u8>> = vec![vec![1; size + 1]; size + 1];

        data.iter().for_each(|&(a, b)| {
            matrix[a][b] = 0;
            matrix[b][a] = 0;
        });

        let mut list: HashMap<usize, Vec<usize>> = HashMap::new();

        for &key in vertices {
            let mut vec: Vec<usize> = Vec::new();
            for i in 1..=size {
                if matrix[key][i] == 1 {
                    vec.push(i);
                }
            }
            vec = vec.iter().filter(|&&val| val != key).copied().collect();
            list.insert(key, vec);
        }
        list.iter_mut().for_each(|(_, val)| val.sort_unstable());
        list
    }

    #[inline]
    fn direct_loop_list(data: &[(usize, usize)], vertices: &[usize]) -> HashMap<usize, Vec<usize>> {
        let mut out: HashMap<usize, Vec<usize>> = HashMap::new();
        data.iter().for_each(|&(first, second)| {
            if let Some(_) = data.iter().find(|&&(a, b)| a == second && b == first) {
                out.entry(first)
                    .and_modify(|loops| loops.push(second))
                    .or_insert_with(|| vec![second]);
            }
        });

        vertices.iter().for_each(|&key| {
            out.entry(key).or_insert_with(|| vec![]);
        });

        out.iter_mut().for_each(|(_, values)| values.sort_unstable());
        out
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_graph_matrix() {
        let graph: Vec<(usize, usize)> = vec![(1, 2), (1, 5), (1, 3), (2, 5), (3, 4), (4, 5)];
        let mut correct_matrix: Vec<Vec<isize>> = Vec::new();
        correct_matrix.push(vec![0,  0,  0,  0,  0,  0,  0,  0,  0,  0]);
        correct_matrix.push(vec![0,  0,  3,  5, -4,  5,  2,  0,  0,  4]);
        correct_matrix.push(vec![0,  7,  0, -4, -4,  5,  5,  1,  0,  3]);
        correct_matrix.push(vec![0,  7, -5,  0,  4, -5,  4,  1,  0,  2]);
        correct_matrix.push(vec![0, -2, -2,  9,  0,  5,  5,  3,  0,  1]);
        correct_matrix.push(vec![0,  8, 10, -3, 10,  0,  0,  1,  0,  3]);
        let generated_matrix = Matrix::from(&graph, 5).matrix();

        eprintln!("{}", pretty_matrix(&correct_matrix));
        eprintln!("{}", pretty_matrix(&generated_matrix));
        assert_eq!(correct_matrix, generated_matrix);
    }
}
