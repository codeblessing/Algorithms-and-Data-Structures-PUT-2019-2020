//! Topological sorting using DFS (Depth First Search) technique and successors list graph representation.
#![allow(dead_code)]
use super::*;
use crate::graph::successors;
use colored::Colorize;
use std::collections::HashMap;

pub fn sort(list: &successors::List) -> Vec<usize> {
    let mut list = to_colored_vertex(&list);

    let mut vertices: Vec<usize> = list.iter().map(|(&key, _)| key).collect();
    vertices.sort_unstable();

    if vertices.is_empty() {
        return vec![];
    }

    let mut sorted: Vec<usize> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();
    let mut vertex;
    stack.push(vertices.remove(0));

    loop {
        // Take vertex from stack and color it grey.
        vertex = stack[stack.len() - 1];
        list.iter_mut().for_each(|(_, val)| {
            val.iter_mut().for_each(|vert| {
                if vert.value == vertex {
                    vert.color(Color::Grey)
                }
            })
        });

        print_colored(&list);
        // If vertex have grey succesor there's cycle and we can't sort this graph.
        if let Some(_) = list[&vertex].iter().find(|val| val.color == Color::Grey) {
            eprintln!("Cannot sort graph with cycles.");
            sorted = vec![];
            break;
        }
        // If vertex have no successors:
        if list[&vertex].is_empty() || list[&vertex].iter().all(|vert| vert.color == Color::Black) {
            // Color it black.
            sorted.push(stack.pop().unwrap());
            list.iter_mut().for_each(|(_, val)| {
                val.iter_mut().for_each(|vert| {
                    if vert.value == vertex {
                        vert.color(Color::Black)
                    }
                })
            });
            // If there's no more vertices to check - end.
            if stack.is_empty() && vertices.is_empty() {
                break;
            }
            // If there's no visited vertices but there are vertices unvisited yet.
            if stack.is_empty() {
                stack.push(vertices.remove(0));
            }
        // If vertex have successors
        } else {
            // Get first successor and color it grey.
            let successor = list[&vertex].iter().find(|val| val.color == Color::White);
            stack.push(vertices.remove(index_of(successor.unwrap().value, &vertices).unwrap()));
        };
    }

    sorted.iter().rev().copied().collect()
}

/// Returns `Some(index)` if key was found or `None` otherwise.
fn index_of(key: usize, vec: &[usize]) -> Option<usize> {
    let results = vec.iter().enumerate().find(|(_, &val)| val == key);
    match results {
        Some((key, _)) => Some(key),
        None => None,
    }
}

/// Transforms successors list from `HashMap<usize, Vec<usize>>`
/// to `HashMap<usize, Vec<ColoredVertex>>` and colors all vertices white.
fn to_colored_vertex(list: &successors::List) -> HashMap<usize, Vec<ColoredVertex>> {
    let mut out: HashMap<usize, Vec<ColoredVertex>> = HashMap::new();
    list.list().iter().for_each(|(&key, values)| {
        out.entry(key).or_insert_with(|| {
            values
                .iter()
                .map(|&value| ColoredVertex {
                    value,
                    color: Color::White,
                })
                .collect()
        });
    });
    out
}

fn print_colored(list: &HashMap<usize, Vec<ColoredVertex>>) {
    for (key, line) in list {
        println!(
            "{}->{}",
            key,
            line.iter()
                .map(|val| {
                    let color = match val.color {
                        Color::White => colored::Color::White,
                        Color::Black => colored::Color::Black,
                        Color::Grey => colored::Color::Cyan,
                    };
                    format!("{:^3}", val.value.to_string().color(color))
                })
                .fold(String::new(), |acc, val| format!("{}, {}", acc, val))
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::graph::successors;
    #[test]
    fn test_sort_successors_list() {
        let graph: Vec<(usize, usize)> = vec![(1, 2), (1, 5), (1, 3), (2, 5), (3, 4), (4, 5)];
        let succ_list = successors::List::from(&graph);
        let sorted = sort(&succ_list);
        assert_eq!(sorted, vec![1, 3, 4, 2, 5])
    }

    #[test]
    fn test_sort_with_cycle() {
        let graph: Vec<(usize, usize)> =
            vec![(1, 5), (1, 3), (2, 1), (2, 5), (3, 2), (3, 4), (4, 5)];
        let succ_list = successors::List::from(&graph);
        let sorted = sort(&succ_list);
        assert_eq!(sorted, vec![])
    }

    #[test]
    fn test_to_colored_vertex() {
        let graph: Vec<(usize, usize)> = vec![(1, 2), (1, 5), (1, 3), (2, 5), (3, 4), (4, 5)];
        let succ_list = successors::List::from(&graph);
        let list = to_colored_vertex(&succ_list);
        for (_, val) in list {
            assert!(val.iter().all(|item| item.color == Color::White));
        }
    }
}
