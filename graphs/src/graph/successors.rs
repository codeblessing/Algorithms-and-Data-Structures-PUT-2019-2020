#![allow(dead_code)]
#![warn(missing_docs)]

//! Module for representing graph as successors list.

use std::collections::HashMap;

/// Represents graph as successors list.
pub struct List {
    list: HashMap<usize, Vec<usize>>,
}

impl List {
    /// Creates successors list from vertex arcs list.
    pub fn from(data: &[(usize, usize)]) -> Self
    {
        let mut list: HashMap<usize, Vec<usize>> = HashMap::new();

        for &(a, b) in data {
            list.entry(a).and_modify(|val| val.push(b)).or_insert_with(|| vec![b]);
        }

        list.iter_mut().for_each(|(_, val)| val.sort_unstable());

        List { list }
    }

    /// Returns copy of successors list as HashMap indexed by vertex number.
    pub fn list(&self) -> HashMap<usize, Vec<usize>> {
        self.list.clone()
    }
}
