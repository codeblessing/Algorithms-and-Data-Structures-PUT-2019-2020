use crate::graph;
use std::sync::mpsc;
use std::{collections::HashMap, thread};

pub fn hamilton_cycles(graph: graph::SuccessorsList) -> Result<Vec<Vec<usize>>, ()> {
    let size = graph.list().len();
    if size < 3 {
        eprintln!("Graf wejściowy jest acykliczny");
        return Err(());
    }

    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    for vert in 1..=size {
        let transmitter = mpsc::Sender::clone(&tx);
        let list = graph.clone();
        let handle = thread::spawn(move || {
            let mut excluded = vec![];
            loop {
                let cycle = find_ham_cycle_from_vertex(list.clone(), vert, &excluded);
                match cycle {
                    Ok(val) => {
                        eprintln!("Thread {}: cycle: {:?}", vert, val);
                        excluded.push(val.clone());
                        transmitter.send(val).unwrap();
                    }
                    Err(_) => break,
                }
            }
        });
        handles.push(handle);
    }
    drop(tx);
    let mut cycles = vec![];
    for msg in rx {
        cycles.push(msg);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    if cycles.is_empty() {
        println!("Graf wejściowy jest acykliczny (nie zawiera cyklu Hamiltona)");
        Err(())
    } else {
        Ok(cycles)
    }
}

/// Searches for Hamilton cycle starting at `vertex` and searching for cycles other than `excluded`.
fn find_ham_cycle_from_vertex(
    graph: graph::SuccessorsList,
    vertex: usize,
    excluded: &Vec<Vec<usize>>,
) -> Result<Vec<usize>, ()> {
    let list = graph.list();
    let size = list.len();

    let mut stack: Vec<usize> = Vec::new();
    let mut exclude: Vec<usize> = Vec::new();
    let mut current: usize = vertex;

    loop {
        // Find successor
        match next(current, &stack, &list, &exclude) {
            // If successor exists and is not on stack yet nor in excluded vertices.
            // Push current vertex to stack, set it's successor as current vertex and clear excluded vertices.
            Some(key) => {
                stack.push(current);
                current = key;
                exclude.clear();
            }
            // If successor doesn't exist or is on stack already.
            // If stack contains all vertices break and check for edge between first and last.
            // If stack is empty (so current vertex is first and has no successors) return.
            // Otherwise set vertices from first to current (because vertices are traversed in increase order)
            // as excluded and set current's predecessor as current vertex.
            None => {
                if stack.len() == size - 1 {
                    stack.push(current);
                    if !excluded.contains(&stack) {
                        break;
                    }
                    stack.pop();
                }
                if stack.is_empty() {
                    return Err(());
                } else {
                    exclude = (1..=current).collect();
                    current = stack.pop().unwrap();
                }
            }
        }
    }

    if check_edge(current, stack[0], &list) {
        Ok(stack)
    } else {
        Err(())
    }
}

/// Searches for free successor of `vertex`. Returns successor if found otherwise returns `None`.
fn next(
    vertex: usize,
    stack: &[usize],
    list: &HashMap<usize, Vec<usize>>,
    exclude: &[usize],
) -> Option<usize> {
    // Get successors, that are not on stack or in excluded.
    let successors: Vec<usize> = list[&vertex]
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

/// Checks whether edge first -> second exists.
fn check_edge(
    first_vert: usize,
    second_vert: usize,
    successors_list: &HashMap<usize, Vec<usize>>,
) -> bool {
    successors_list[&first_vert].contains(&second_vert)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_all_ham_cycles() {
        let mut list: HashMap<usize, Vec<usize>> = HashMap::new();
        list.insert(1, vec![2]);
        list.insert(2, vec![3]);
        list.insert(3, vec![1]);

        let list = graph::SuccessorsList::from(list);

        let cycles = hamilton_cycles(list).unwrap();

        for (key, cycle) in cycles.iter().enumerate() {
            eprintln!("Cycle {}: {:?}", key, cycle);
        }

        assert!(cycles.iter().any(|v| *v == vec![1, 2, 3]));
        assert!(cycles.iter().any(|v| *v == vec![2, 3, 1]));
        assert!(cycles.iter().any(|v| *v == vec![3, 1, 2]));
    }
    #[test]
    fn test_find_ham_cycle() {
        let mut list: HashMap<usize, Vec<usize>> = HashMap::new();
        list.insert(1, vec![2]);
        list.insert(2, vec![3]);
        list.insert(3, vec![1]);

        let list = graph::SuccessorsList::from(list);
        let excluded: Vec<Vec<usize>> = Vec::new();

        let cycle = find_ham_cycle_from_vertex(list.clone(), 1, &excluded).unwrap();

        assert_eq!(cycle, vec![1, 2, 3]);
    }

    #[test]
    fn test_no_ham_cycle() {
        let mut list: HashMap<usize, Vec<usize>> = HashMap::new();
        list.insert(1, vec![2]);
        list.insert(2, vec![3]);
        list.insert(3, vec![]);

        let list = graph::SuccessorsList::from(list);

        let cycle = find_ham_cycle_from_vertex(list, 1, &vec![]);

        assert_eq!(cycle, Err(()));
    }
}
