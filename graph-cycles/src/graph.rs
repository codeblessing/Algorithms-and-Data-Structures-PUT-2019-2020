#![warn(clippy::all)]
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

#[derive(Clone)]
pub struct AdjacencyMatrix {
    matrix: Vec<Vec<u8>>,
}

impl AdjacencyMatrix {
    pub fn new() -> Self {
        Self { matrix: vec![] }
    }

    /// Returns first successor of `node`.
    /// If `node` have no successors returns `None`
    pub fn next(&self, node: usize) -> Option<usize> {
        match self.matrix[node]
            .iter()
            .enumerate()
            .find(|&(_, &val)| val == 1)
        {
            Some((key, _)) => Some(key),
            None => None,
        }
    }

    /// Removes edge between `from` and `to` from adjacency matrix.
    /// # Returns
    /// If given edge exists removes it and returns `Ok(())`.
    /// Otherwise returns `Err(())`.
    pub fn remove_edge(&mut self, from: usize, to: usize) -> Result<(), ()> {
        if self.matrix[from][to] == 0 || self.matrix[to][from] == 0 {
            Err(())
        } else {
            self.matrix[from][to] = 0;
            self.matrix[to][from] = 0;
            Ok(())
        }
    }

    /// Returns degree of `node` (count of edges connected with `node`).
    pub fn deg(&self, node: usize) -> usize {
        self.matrix[node].iter().filter(|&&val| val == 1).count()
    }

    pub fn has_edges(&self) -> bool {
        self.matrix
            .iter()
            .any(|edges| edges.iter().any(|&vert| vert == 1))
    }

    /// Returns adjacency matrix as two-dimensional `Vec`.
    pub fn matrix(&self) -> Vec<Vec<u8>> {
        self.matrix.clone()
    }
}

impl From<Vec<Vec<u8>>> for AdjacencyMatrix {
    fn from(input: Vec<Vec<u8>>) -> Self {
        let mut matrix: Vec<Vec<u8>> = Vec::new();
        for row in input {
            matrix.push(row.iter().copied().collect::<Vec<u8>>());
        }

        Self { matrix }
    }
}

impl From<&[(usize, usize)]> for AdjacencyMatrix {
    fn from(arcs: &[(usize, usize)]) -> Self {
        let v_count = arcs
            .iter()
            .fold(HashSet::new(), |mut acc, (a, b)| {
                acc.insert(a);
                acc.insert(b);
                acc
            })
            .len()
            + 1;

        let mut matrix: Vec<Vec<u8>> = vec![vec![0; v_count]; v_count];

        for &(from, to) in arcs {
            matrix[from][to] = 1;
            matrix[to][from] = 1;
        }

        Self { matrix }
    }
}

#[derive(Clone)]
pub struct SuccessorsList {
    list: HashMap<usize, Vec<usize>>,
}

impl SuccessorsList {
    /// Creates new, empty successors list.
    pub fn new() -> Self {
        Self {
            list: HashMap::new(),
        }
    }

    /// Returns first successor of `node`.
    /// If `node` have no successors returns `None`
    pub fn next(&self, node: usize) -> Option<usize> {
        if self.list[&node].is_empty() {
            None
        } else {
            Some(self.list[&node][0])
        }
    }

    pub fn has_edges(&self) -> bool {
        self.list
            .iter()
            .any(|(_, successors)| !successors.is_empty())
    }

    /// Removes edge `from` -> `to` from Successors List.
    /// # Returns
    /// If `to` is successor of `from` removes it and returns `Ok(())`.
    /// If `to` is not successor of `from` returns `Err(())`.
    pub fn remove_edge(&mut self, from: usize, to: usize) -> Result<(), ()> {
        if self.list[&from].contains(&to) {
            self.list.insert(
                from,
                self.list[&from]
                    .iter()
                    .filter(|&&vert| vert != to)
                    .copied()
                    .collect(),
            );
            Ok(())
        } else {
            Err(())
        }
    }

    /// Removes `node` occurences in graph.

    /// Returns `node`\`s output degree (number of arcs starting at `node`)
    pub fn deg_out(&self, node: usize) -> usize {
        self.list[&node].len()
    }

    /// Returns `node`\`s input degree (number of arcs ending at `node`)
    pub fn deg_in(&self, node: usize) -> usize {
        let mut count: usize = 0;
        self.list.iter().for_each(|(_, succ)| {
            if succ.contains(&node) {
                count += 1;
            }
        });
        count
    }

    /// Returns in/out edges balance.
    /// If node has equal in and out degrees returns 0
    /// If node has greater in degree than out degree returns -1
    /// If node has greater out degree than in degree returns 1
    pub fn deg(&self, node: usize) -> i8 {
        match self.deg_in(node).cmp(&self.deg_out(node)) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => 0,
        }
    }

    /// Returns successors list as `HashMap` indexed by vertex number (must be `usize`)
    pub fn list(&self) -> HashMap<usize, Vec<usize>> {
        self.list.clone()
    }
}

impl From<HashMap<usize, Vec<usize>>> for SuccessorsList {
    fn from(input: HashMap<usize, Vec<usize>>) -> Self {
        Self { list: input }
    }
}

impl From<&[(usize, usize)]> for SuccessorsList {
    fn from(arcs: &[(usize, usize)]) -> Self {
        let mut list: HashMap<usize, Vec<usize>> = HashMap::new();
        for &(from, to) in arcs {
            list.entry(from)
                .and_modify(|entry| {
                    if !entry.contains(&to) {
                        entry.push(to)
                    }
                })
                .or_insert_with(|| vec![to]);
        }
        Self { list }
    }
}
