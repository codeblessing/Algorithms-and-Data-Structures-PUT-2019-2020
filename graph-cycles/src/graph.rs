#![warn(clippy::all)]
use log::warn;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq)]
pub struct AdjacencyMatrix {
    matrix: Vec<Vec<u8>>,
    nodes: HashSet<usize>,
}

impl AdjacencyMatrix {
    pub fn new() -> Self {
        Self {
            matrix: Vec::new(),
            nodes: HashSet::new(),
        }
    }

    /// Returns first successor of `node`.
    /// If `node` have no successors or is not correct node returns `None`
    pub fn next(&self, node: usize) -> Option<usize> {
        if !self.nodes.contains(&node) {
            warn!("Wierzchołek {} nie należy do grafu", node);
            return None;
        }

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
    pub fn remove_edge(&mut self, from: usize, to: usize) {
        if !self.nodes.contains(&from) || !self.nodes.contains(&to) {
            warn!("{} lub {} nie należy do grafu", from, to);
        } else {
            self.matrix[from][to] = 0;
            self.matrix[to][from] = 0;
        }
    }

    /// Returns degree of `node` (count of edges connected with `node`) or `None` if node doesn't belongs to graph.
    pub fn deg(&self, node: usize) -> Option<usize> {
        if !self.nodes.contains(&node) {
            warn!("{} nie jest wierzchołkiem grafu.", node);
            None
        } else {
            Some(self.matrix[node].iter().filter(|&&val| val == 1).count())
        }
    }

    pub fn has_edges(&self) -> bool {
        self.matrix
            .iter()
            .any(|edges| edges.iter().any(|&vert| vert == 1))
    }

    /// Returns adjacency matrix as two-dimensional `Vec`.
    pub fn matrix_mut(&self) -> Vec<Vec<u8>> {
        self.matrix.clone()
    }

    /// Returns reference to adjacency matrix as two-dimensional `Vec`.
    pub fn matrix(&self) -> &Vec<Vec<u8>> {
        &self.matrix
    }

    /// Returns reference to nodes.
    pub fn nodes(&self) -> &HashSet<usize> {
        &self.nodes
    }
}

impl<B> From<B> for AdjacencyMatrix
where
    B: Into<Vec<(usize, usize)>>,
{
    fn from(arcs: B) -> Self {
        let mut arcs: Vec<(usize, usize)> = arcs.into();
        let node_count = arcs.remove(0).0;
        let nodes: HashSet<usize> = (1..=node_count).collect();

        let nodes_count = nodes.len() + 1;

        let mut matrix: Vec<Vec<u8>> = vec![vec![0; nodes_count]; nodes_count];

        for (from, to) in arcs {
            matrix[from][to] = 1;
            matrix[to][from] = 1;
        }

        Self { matrix, nodes }
    }
}

#[derive(Clone, Debug, PartialEq)]
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
    /// If `node` have no successors or doesn't belongs to graph returns `None`
    pub fn next(&self, node: usize) -> Option<usize> {
        if !self.list.contains_key(&node) {
            warn!("{} nie jest wierzchokiem grafu", node);
            None
        } else {
            if self.list[&node].is_empty() {
                None
            } else {
                Some(self.list[&node][0])
            }
        }
    }

    pub fn has_edges(&self) -> bool {
        self.list
            .iter()
            .any(|(_, successors)| !successors.is_empty())
    }

    /// Removes edge `from` -> `to` from Successors List.
    pub fn remove_edge(&mut self, from: usize, to: usize) {
        if !self.list.contains_key(&from) || !self.list.contains_key(&to) {
            warn!("{} -> {} nie jest krawędzią grafu", from, to);
        } else {
            if self.list[&from].contains(&to) {
                self.list.insert(
                    from,
                    self.list[&from]
                        .iter()
                        .filter(|&&vert| vert != to)
                        .copied()
                        .collect(),
                );
            } else {
                warn!("{} -> {} nie jest krawędzią grafu", from, to);
            }
        }
    }

    /// Returns (out - in) edges difference.
    /// If it's 0 then node's output edge count is equal it's input edge count.
    pub fn deg(&self, node: usize) -> Option<isize> {
        if !self.list.contains_key(&node) {
            None
        } else {
            let deg_out = self.list[&node].len() as isize;
            let mut deg_in: isize = 0;
            self.list.iter().for_each(|(_, succ)| {
                if succ.contains(&node) {
                    deg_in += 1;
                }
            });
            Some(deg_out - deg_in)
        }
    }

    /// Returns successors list as `HashMap` indexed by vertex number (must be `usize`)
    pub fn list_mut(&self) -> HashMap<usize, Vec<usize>> {
        self.list.clone()
    }

    /// Returns successors list as immutable reference to `HashMap` indexed by vertex number.
    pub fn list(&self) -> &HashMap<usize, Vec<usize>> {
        &self.list
    }
}

impl<B> From<B> for SuccessorsList
where
    B: Into<Vec<(usize, usize)>>,
{
    fn from(arcs: B) -> Self {
        let mut arcs: Vec<(usize, usize)> = arcs.into();

        let node_count = arcs.remove(0).0;
        let nodes: HashSet<usize> = (1..=node_count).collect();

        let mut list: HashMap<usize, Vec<usize>> = HashMap::new();
        for (from, to) in arcs {
            list.entry(from)
                .and_modify(|entry| {
                    if !entry.contains(&to) {
                        entry.push(to)
                    }
                })
                .or_insert_with(|| vec![to]);
        }

        for node in nodes {
            list.entry(node).or_insert_with(|| Vec::new());
        }

        Self { list }
    }
}

#[cfg(test)]
mod test_adjacency_matrix {
    use super::*;

    #[test]
    fn test_create_empty() {
        assert_eq!(
            AdjacencyMatrix::new(),
            AdjacencyMatrix {
                matrix: Vec::new(),
                nodes: HashSet::new()
            }
        );
    }

    #[test]
    fn test_create_from() {
        let matrix = AdjacencyMatrix {
            matrix: vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 1, 0],
                vec![0, 1, 0, 1],
                vec![0, 0, 1, 0],
            ],

            nodes: (1..=3).collect(),
        };
        assert_eq!(AdjacencyMatrix::from(vec![(3, 2), (1, 2), (2, 3)]), matrix);
    }

    #[test]
    fn test_get_next() {
        let matrix = AdjacencyMatrix::from(vec![(4, 5), (1, 2), (2, 3), (2, 4), (3, 4), (4, 1)]);
        assert_eq!(matrix.next(1), Some(2));
        assert_eq!(matrix.next(2), Some(1));
        assert_eq!(matrix.next(4), Some(1));
        assert_eq!(matrix.next(5), None);
    }

    #[test]
    fn test_get_next_from_empty() {
        assert_eq!(AdjacencyMatrix::new().next(0), None);
        assert_eq!(AdjacencyMatrix::new().next(1), None);
    }

    #[test]
    fn test_remove_edge_from_matrix() {
        let mut matrix = AdjacencyMatrix::from(vec![(3, 4), (1, 2), (1, 3), (2, 3), (3, 1)]);
        assert_eq!(matrix.next(1), Some(2));
        matrix.remove_edge(1, 2);
        assert_eq!(matrix.next(1), Some(3));
    }

    #[test]
    fn test_check_node_degree() {
        let matrix =
            AdjacencyMatrix::from(vec![(5, 6), (1, 2), (1, 3), (2, 3), (3, 4), (3, 5), (4, 5)]);
        assert_eq!(matrix.deg(1), Some(2));
        assert_eq!(matrix.deg(2), Some(2));
        assert_eq!(matrix.deg(3), Some(4));
        assert_eq!(matrix.deg(4), Some(2));
        assert_eq!(matrix.deg(5), Some(2));
    }

    #[test]
    fn test_has_edges() {
        let no_edges = AdjacencyMatrix {
            matrix: vec![
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
            ],
            nodes: (1..=4).collect(),
        };

        let empty_matrix = AdjacencyMatrix::new();
        let matrix = AdjacencyMatrix::from(vec![(5, 5), (1, 2), (2, 3), (2, 5), (3, 4), (3, 5)]);

        assert!(!no_edges.has_edges());
        assert!(!empty_matrix.has_edges());
        assert!(matrix.has_edges());
    }
}

#[cfg(test)]
mod test_successors_list {
    use super::*;

    #[test]
    fn test_create_empty() {
        assert_eq!(
            SuccessorsList::new(),
            SuccessorsList {
                list: HashMap::new()
            }
        );
    }

    #[test]
    fn test_create_from() {
        let mut list = HashMap::new();
        list.insert(1, vec![2, 3]);
        list.insert(2, vec![3]);
        list.insert(3, vec![1, 4]);
        list.insert(4, vec![]);

        let list = SuccessorsList { list };

        assert_eq!(
            SuccessorsList::from(vec![(4, 5), (1, 2), (1, 3), (2, 3), (3, 1), (3, 4)]),
            list
        );
    }

    #[test]
    fn test_get_next() {
        let list = SuccessorsList::from(vec![(4, 5), (1, 2), (2, 3), (2, 4), (3, 4), (4, 1)]);

        assert_eq!(list.next(1), Some(2));
        assert_eq!(list.next(2), Some(3));
        assert_eq!(list.next(3), Some(4));
        assert_eq!(list.next(4), Some(1));
        assert_eq!(list.next(5), None)
    }

    #[test]
    fn test_get_next_from_empty() {
        let list = SuccessorsList::new();
        assert_eq!(list.next(1), None);
    }

    #[test]
    fn test_remove_edge_from_list() {
        let mut list = SuccessorsList::from(vec![(3, 4), (1, 2), (1, 3), (2, 3), (3, 1)]);
        assert_eq!(list.next(1), Some(2));
        list.remove_edge(1, 2);
        assert_eq!(list.next(1), Some(3));
    }

    #[test]
    fn test_check_node_degree() {
        let list = SuccessorsList::from(vec![(5, 6), (1, 2), (1, 3), (2, 3), (3, 4), (3, 5), (4, 5)]);
        assert_eq!(list.deg(1), Some(2));
        assert_eq!(list.deg(2), Some(0));
        assert_eq!(list.deg(3), Some(0));
        assert_eq!(list.deg(4), Some(0));
        assert_eq!(list.deg(5), Some(-2));
        assert_eq!(list.deg(8), None);
    }

    #[test]
    fn test_has_edges() {
        let list = SuccessorsList::from(vec![(3, 3), (1, 2), (2, 3), (3, 1)]);
        let no_edges = SuccessorsList {
            list: (0..5).map(|key| (key, Vec::with_capacity(2))).collect(),
        };
        let empty = SuccessorsList::new();

        assert!(list.has_edges());
        assert!(!no_edges.has_edges());
        assert!(!empty.has_edges());
    }
}
