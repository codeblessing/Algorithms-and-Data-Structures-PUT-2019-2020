#![warn(missing_docs)]

//! Module for representing graph as successors list.
use std::collections::{HashMap, HashSet};

/// Represents graph as successors list.
pub struct List {
    list: HashMap<usize, Vec<usize>>,
}

impl List {
    /// Creates successors list from vertex arcs list.
    pub fn from(data: &[(usize, usize)]) -> Self {
        let vertices: HashSet<usize> = data.iter().fold(HashSet::new(), |mut acc, &(a, b)| {
            acc.insert(a);
            acc.insert(b);
            acc
        });

        let mut list: HashMap<usize, Vec<usize>> = HashMap::new();

        for &(a, b) in data {
            list.entry(a)
                .and_modify(|val| val.push(b))
                .or_insert_with(|| vec![b]);
        }

        vertices.iter().for_each(|&key| {
            list.entry(key).or_insert_with(|| vec![]);
        });

        list.iter_mut().for_each(|(_, val)| val.sort_unstable());

        List { list }
    }

    /// Returns copy of successors list as HashMap indexed by vertex number.
    pub fn list(&self) -> HashMap<usize, Vec<usize>> {
        self.list.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_create_successors_list() {
        let input: Vec<(usize, usize)> = vec![(1, 2), (1, 3), (1, 5), (2, 5), (3, 4), (4, 5)];
        let mut correct_list: HashMap<usize, Vec<usize>> = HashMap::new();
        correct_list.insert(1, vec![2, 3, 5]);
        correct_list.insert(2, vec![5]);
        correct_list.insert(3, vec![4]);
        correct_list.insert(4, vec![5]);
        correct_list.insert(5, vec![]);
        let generated_list = List::from(&input).list();
        assert_eq!(correct_list, generated_list);
    }
}
