// #![allow(dead_code)]
#![warn(clippy::all)]
//! Module for representing graph as graph matrix.

pub mod neighbourhood;
pub mod successors;

use std::collections::HashMap;
use std::collections::HashSet;

/// Represents graph as graph matrix.
pub struct Matrix {
    matrix: Vec<Vec<isize>>,
}

impl Matrix {
    /// Creates graph matrix from vertex arcs list.
    /// # Panics
    /// Panics when `max` is incorrect (and triggers index out of range).
    pub fn from(data: &[(usize, usize)], max: usize) -> Self {
        let vertices: HashSet<usize> = data.iter().fold(HashSet::new(), |mut acc, val| {
            acc.insert(val.0);
            acc.insert(val.1);
            acc
        });
        let mut matrix: Vec<Vec<isize>> = vec![vec![0; max + 5]; max + 1];
        let successors = Matrix::succ_list(data);
        let predecessors = Matrix::pred_list(data);
        let no_consecutive = Matrix::no_consecutive_list(data);

        successors
            .iter()
            .for_each(|(&a, b)| matrix[a][max + 1] = b[0] as isize);

        vertices.iter().for_each(|&val| {
            let succ = &successors[&val];
            let last = succ[succ.len() - 1];
            succ.iter()
                .for_each(|&ind| matrix[val][ind] = last as isize);
        });

        predecessors
            .iter()
            .for_each(|(&a, b)| matrix[a][max + 2] = b[0] as isize);

        vertices.iter().for_each(|&val| {
            let pred = &predecessors[&val];
            let &last = pred.iter().last().unwrap();
            pred.iter()
                .for_each(|&ind| matrix[val][ind] = (last + max) as isize);
        });

        no_consecutive
            .iter()
            .for_each(|(&a, b)| matrix[a][max + 3] = b[0] as isize);
        vertices.iter().for_each(|&val| {
            let no_cons = &no_consecutive[&val];
            let &last = no_cons.iter().last().unwrap();
            no_cons
                .iter()
                .for_each(|&ind| matrix[val][ind] = -(last as isize));
        });
        Matrix { matrix }
    }

    /// Returns graph matrix as Vec<Vec<isize>>.
    pub fn matrix(&self) -> Vec<Vec<isize>> {
        self.matrix.clone()
    }

    #[inline]
    fn succ_list(data: &[(usize, usize)]) -> HashMap<usize, Vec<usize>> {
        let mut list: HashMap<usize, Vec<usize>> = HashMap::new();

        data.iter().for_each(|&(a, b)| {
            list.entry(a)
                .and_modify(|val| val.push(b))
                .or_insert_with(|| vec![b]);
        });

        list.iter_mut().for_each(|(_, val)| val.sort_unstable());

        list
    }

    #[inline]
    fn pred_list(data: &[(usize, usize)]) -> HashMap<usize, Vec<usize>> {
        let mut list: HashMap<usize, Vec<usize>> = HashMap::new();

        data.iter().for_each(|&(a, b)|{
            list.entry(b)
                .and_modify(|val| val.push(a))
                .or_insert_with(|| vec![a]);
        });

        list.iter_mut().for_each(|(_, val)| val.sort_unstable());

        list
    }

    #[inline]
    fn no_consecutive_list(data: &[(usize, usize)]) -> HashMap<usize, Vec<usize>> {
        let all: HashSet<usize> = data.iter().fold(HashSet::new(), |mut acc, val| {
            acc.insert(val.0);
            acc.insert(val.1);
            acc
        });

        if all.is_empty() {
            return HashMap::new();
        }

        let &size = all.iter().max().unwrap();

        let mut matrix: Vec<Vec<u8>> = vec![vec![1; size + 1]; size + 1];

        data.iter().for_each(|&(a, b)| matrix[a][b] = 0);

        let mut list: HashMap<usize, Vec<usize>> = HashMap::new();

        for key in all {
            let mut vec: Vec<usize> = Vec::new();
            for i in 0..=size {
                if matrix[key][i] == 1 {
                    vec.push(i);
                }
            }
            list.insert(key, vec);
        }
        list.iter_mut().for_each(|(_, val)| val.sort_unstable());
        list
    }
}
