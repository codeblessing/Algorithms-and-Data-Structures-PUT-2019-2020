use crate::graph::SuccessorsList;
use std::sync::mpsc;
use std::thread;
use log::debug;

pub fn hamilton_cycle(graph: SuccessorsList) -> Option<Vec<usize>> {
    let size = graph.list().len();
    if size < 3 {
        eprintln!("Graf wejściowy jest acykliczny (nie zawiera cyklu Hamiltona)");
        return None;
    }

    let (tx, rx) = mpsc::channel();

    for vert in 1..=size {
        let transmitter = mpsc::Sender::clone(&tx);
        let list = graph.clone();
        thread::spawn(move || {
            loop {
                let cycle = cycle_from_node(list.clone(), vert);
                match cycle {
                    Some(val) => {
                        debug!("Thread {}: cycle: {:?}", vert, val);
                        transmitter.send(val).unwrap_or(());
                    }
                    None => break,
                }
            }
        });
    }

    drop(tx);
    let mut cycle = None;
    for msg in rx {
        cycle = Some(msg);
        break;
    }

    if let None = cycle {
        println!("Graf wejściowy jest acykliczny (nie zawiera cyklu Hamiltona)");
        None
    } else {
        cycle
    }
}

/// Searches for Hamilton cycle starting at `node` and searching for cycles other than `excluded`.
fn cycle_from_node(graph: SuccessorsList, node: usize) -> Option<Vec<usize>> {
    let size = graph.list().len();

    let mut stack: Vec<usize> = Vec::new();
    let mut exclude: Vec<usize> = Vec::new();
    let mut current: usize = node;

    loop {
        // Find successor
        match next(current, &stack, &graph, &exclude) {
            // If successor exists and is not on stack yet nor in excluded vertices.
            // Push current node to stack, set it's successor as current node and clear excluded vertices.
            Some(key) => {
                stack.push(current);
                current = key;
                exclude.clear();
            }
            // If successor doesn't exist or is on stack already.
            // If stack contains all vertices break and check for edge between first and last.
            // If stack is empty (so current node is first and has no successors) return.
            // Otherwise step back. [set vertices from first to current (because vertices are traversed in increase order)
            // as excluded and set current's predecessor as current node].
            None => {
                if stack.len() == size - 1 {
                    stack.push(current);
                    break;
                }
                if stack.is_empty() {
                    return None;
                } else {
                    exclude = (1..=current).collect();
                    current = stack.pop().unwrap();
                }
            }
        }
    }

    if check_edge(current, stack[0], &graph) {
        Some(stack)
    } else {
        None
    }
}

/// Searches for free successor of `node`. Returns successor if found otherwise returns `None`.
fn next(node: usize, stack: &[usize], list: &SuccessorsList, exclude: &[usize]) -> Option<usize> {
    if !list.list().contains_key(&node) {
        None
    } else {
        // Get successors, that are not on stack or in excluded.
        let successors: Vec<usize> = list.list()[&node]
            .iter()
            .filter(|&val| !stack.contains(val) && !exclude.contains(val))
            .copied()
            .collect();

        // If there are such successors return first of them else return None.
        if successors.len() > 0 {
            Some(successors[0])
        } else {
            None
        }
    }
}

/// Checks whether edge first -> second exists.
fn check_edge(from: usize, to: usize, list: &SuccessorsList) -> bool {
    if !list.list().contains_key(&from) || !list.list().contains_key(&to) {
        false
    } else {
        list.list()[&from].contains(&to)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::graph::SuccessorsList;
    use log::info;

    #[test]
    fn test_find_hamilton_cycle() {
        let list = SuccessorsList::from(vec![
            (1, 2),
            (2, 3),
            (2, 4),
            (3, 4),
            (3, 5),
            (4, 1),
            (4, 5),
            (5, 1),
        ]);
        let cycle = hamilton_cycle(list);
        assert_ne!(cycle, None);
        eprintln!("[TEST] Found cycle: {:?}", cycle);
    }

    #[test]
    fn test_run_with_acyclic_graph() {
        let list = SuccessorsList::from(vec![(1, 2), (1, 3), (1, 4), (1, 5), (2, 3), (2, 4), (2, 5), (3, 4), (3, 5), (4, 5)]);
        let cycle = hamilton_cycle(list);
        assert_eq!(cycle, None);
    }

    #[test]
    fn test_graph_with_two_nodes() {
        let list = SuccessorsList::from(vec![(1, 2), (2, 1)]);
        let cycle = hamilton_cycle(list);
        assert_eq!(cycle, None);
    }

    #[test]
    fn test_with_empty_graph() {
        let list = SuccessorsList::new();
        let cycle = hamilton_cycle(list);
        assert_eq!(cycle, None);
    }
}
