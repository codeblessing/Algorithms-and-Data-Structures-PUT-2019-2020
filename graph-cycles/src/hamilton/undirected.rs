use crate::graph::AdjacencyMatrix;
use std::sync::mpsc;
use std::thread;

pub fn hamilton_cycle(graph: AdjacencyMatrix) -> Option<Vec<usize>> {
    let size = graph.matrix().len();
    if size < 4 {
        eprintln!("Graf wejściowy jest acykliczny");
        return None;
    }

    let (tx, rx) = mpsc::channel();

    for vert in 1..size {
        let transmitter = mpsc::Sender::clone(&tx);

        let matrix = graph.clone();
        thread::spawn(move || {
            let cycle = cycle_from_vertex(matrix.clone(), vert);
            if let Some(val) = cycle {
                transmitter.send(val).unwrap_or(());
            }
        });
    }

    drop(tx);

    let mut cycle = None;

    for msg in rx {
        cycle = Some(msg);
        break;
    }

    if cycle.is_none() {
        println!("Graf wejściowy jest acykliczny (nie zawiera cyklu Hamiltona)");
        None
    } else {
        cycle
    }
}

/// Searches for Hamilton cycle starting at `vertex` and searching for cycles other than `excluded`.
fn cycle_from_vertex(graph: AdjacencyMatrix, vertex: usize) -> Option<Vec<usize>> {
    let size = graph.nodes().len();

    let mut stack: Vec<usize> = Vec::new();
    let mut excluded: Vec<usize> = Vec::new();
    let mut current: usize = vertex;

    loop {
        // Find successor
        match next(current, &stack, &graph, &excluded) {
            // If successor exists and is not on stack yet nor in excluded vertices.
            // Push current vertex to stack, set it's successor as current vertex and clear excluded vertices.
            Some(key) => {
                stack.push(current);
                current = key;
                excluded.clear();
            }
            // If successor doesn't exist or is on stack already.
            // If stack contains all vertices break and check for edge between first and last (line 97).
            // If stack is empty (so current vertex is first and has no successors) return (line 87).
            // Otherwise set vertices from first to current (because vertices are traversed in increase order)
            // as excluded and set current's predecessor as current vertex (line 89).
            None => {
                if stack.len() == size - 1 {
                    stack.push(current);
                    break;
                }
                if stack.is_empty() {
                    return None;
                } else {
                    excluded = (1..=current).collect();
                    current = stack.pop().unwrap();
                }
            }
        }
    }

    if check_edge(stack[0], current, &graph) {
        Some(stack)
    } else {
        None
    }
}

/// Searches for free successor of `vertex`. Returns successor if found otherwise returns `None`.
fn next(vertex: usize, stack: &[usize], graph: &AdjacencyMatrix, excluded: &[usize]) -> Option<usize> {
    // Get successors, that are not on stack or in excluded.
    if !graph.nodes().contains(&vertex) {
        None
    } else {
        let successors: Vec<_> = graph.matrix()[vertex]
            .iter()
            .enumerate()
            .filter(|&(key, &val)| val == 1 && !stack.contains(&key) && !excluded.contains(&key))
            .collect();

        // If there are such successors return first of them else return None.
        if successors.len() > 0 {
            Some(successors[0].0)
        } else {
            None
        }
    }
}

/// Checks whether edge between given vertices exists.
fn check_edge(from: usize, to: usize, graph: &AdjacencyMatrix) -> bool {
    if !graph.nodes().contains(&from) || !graph.nodes().contains(&to) {
        false
    } else {
        graph.matrix()[from][to] == 1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::graph::AdjacencyMatrix;

    #[test]
    fn test_find_hamilton_cycle() {
        let arcs = vec![(1, 2), (1, 3), (2, 1), (2, 3), (3, 1), (3, 2)];

        let matrix = AdjacencyMatrix::from(arcs);

        let cycle = hamilton_cycle(matrix);

        assert_ne!(cycle, None);
        eprintln!("{:?}", cycle);
    }

    #[test]
    fn test_run_with_acyclic_graph() {
        let arcs = vec![
            (1, 2),
            (1, 3),
            (2, 4),
            (2, 5),
            (3, 6),
            (3, 7),
        ];
        let matrix = AdjacencyMatrix::from(arcs);

        let cycle = hamilton_cycle(matrix);

        assert_eq!(cycle, None);
    }

    #[test]
    fn test_no_hamilton_cycle() {
        let arcs = vec![(1, 2), (2, 3), (3, 4), (4, 5)];
        let matrix = AdjacencyMatrix::from(arcs);

        let cycle = hamilton_cycle(matrix);

        assert_eq!(cycle, None);
    }

    #[test]
    fn test_run_with_empty_graph() {
        let matrix = AdjacencyMatrix::new();
        let cycle = hamilton_cycle(matrix);

        assert_eq!(cycle, None);
    }
}
