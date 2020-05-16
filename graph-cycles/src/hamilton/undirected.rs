// use crate::graph;
// use std::sync::mpsc;
// use std::thread;

// pub fn hamilton_cycles(graph: graph::AdjacencyMatrix) -> Result<Vec<Vec<usize>>, ()> {
//     let size = graph.matrix().len();
//     if size < 4 {
//         eprintln!("Graf wejściowy jest acykliczny");
//         return Err(());
//     }

//     let (tx, rx) = mpsc::channel();
//     let mut handles = vec![];

//     for vert in 1..size {
//         let transmitter = mpsc::Sender::clone(&tx);
//         let matrix = graph.clone();
//         let handle = thread::spawn(move || {
//             let mut excluded = vec![];
//             loop {
//                 let cycle = find_ham_cycle_from_vertex(matrix.clone(), vert, &excluded);
//                 match cycle {
//                     Ok(val) => {
//                         excluded.push(val.clone());
//                         transmitter.send(val).unwrap();
//                     }
//                     Err(_) => break,
//                 }
//             }
//         });
//         handles.push(handle);
//     }
//     drop(tx);

//     let mut cycles = vec![];
    
//     for msg in rx {
//         cycles.push(msg);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     if cycles.is_empty() {
//         println!("Graf wejściowy jest acykliczny (nie zawiera cyklu Hamiltona)");
//         Err(())
//     } else {
//         Ok(cycles)
//     }
// }

// /// Searches for Hamilton cycle starting at `vertex` and searching for cycles other than `excluded`.
// fn find_ham_cycle_from_vertex(
//     graph: graph::AdjacencyMatrix,
//     vertex: usize,
//     excluded: &Vec<Vec<usize>>,
// ) -> Result<Vec<usize>, ()> {
//     let matrix = graph.matrix();
//     let size = matrix.len() - 1;

//     let mut stack: Vec<usize> = Vec::new();
//     let mut exclude: Vec<usize> = Vec::new();
//     let mut current: usize = vertex;

//     loop {
//         // Find successor
//         match next(current, &stack, &matrix, &exclude) {
//             // If successor exists and is not on stack yet nor in excluded vertices.
//             // Push current vertex to stack, set it's successor as current vertex and clear excluded vertices.
//             Some(key) => {
//                 stack.push(current);
//                 current = key;
//                 exclude.clear();
//             }
//             // If successor doesn't exist or is on stack already.
//             // If stack contains all vertices break and check for edge between first and last (line 97).
//             // If stack is empty (so current vertex is first and has no successors) return (line 87).
//             // Otherwise set vertices from first to current (because vertices are traversed in increase order)
//             // as excluded and set current's predecessor as current vertex (line 89).
//             None => {
//                 if stack.len() == size - 1 {
//                     stack.push(current);
//                     if !excluded.contains(&stack) {
//                         break;
//                     }
//                     stack.pop();
//                 }
//                 if stack.is_empty() {
//                     return Err(());
//                 } else {
//                     exclude = (1..=current).collect();
//                     current = stack.pop().unwrap();
//                 }
//             }
//         }
//     }

//     if check_edge(stack[0], current, &matrix) {
//         Ok(stack)
//     } else {
//         Err(())
//     }
// }

// /// Searches for free successor of `vertex`. Returns successor if found otherwise returns `None`.
// fn next(vertex: usize, stack: &[usize], matrix: &Vec<Vec<u8>>, exclude: &[usize]) -> Option<usize> {
//     // Get successors, that are not on stack or in excluded.
//     let successors: Vec<_> = matrix[vertex]
//         .iter()
//         .enumerate()
//         .filter(|&(key, &val)| val == 1 && !stack.contains(&key) && !exclude.contains(&key))
//         .collect();

//     // If there are such successors return first of them else return None.
//     if successors.len() > 0 {
//         Some(successors[0].0)
//     } else {
//         None
//     }
// }

// /// Checks whether edge between given vertices exists.
// fn check_edge(first_vert: usize, second_vert: usize, adjacency_matrix: &Vec<Vec<u8>>) -> bool {
//     adjacency_matrix[first_vert][second_vert] == 1
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_find_all_ham_cycles() {
//         let matrix: Vec<Vec<u8>> = vec![
//             vec![0, 0, 0, 0],
//             vec![0, 0, 1, 1],
//             vec![0, 1, 0, 1],
//             vec![0, 1, 1, 0],
//         ];

//         let matrix = graph::AdjacencyMatrix::from(matrix);

//         let cycles = hamilton_cycles(matrix).unwrap();

//         assert!(cycles.iter().any(|v| *v == vec![1, 2, 3]));
//         assert!(cycles.iter().any(|v| *v == vec![1, 3, 2]));
//         assert!(cycles.iter().any(|v| *v == vec![2, 1, 3]));
//         assert!(cycles.iter().any(|v| *v == vec![2, 3, 1]));
//         assert!(cycles.iter().any(|v| *v == vec![3, 1, 2]));
//         assert!(cycles.iter().any(|v| *v == vec![3, 2, 1]));

//         for (key, cycle) in cycles.iter().enumerate() {
//             eprintln!("Cycle {}: {:?}", key, cycle);
//         }
//     }
//     #[test]
//     fn test_find_ham_cycle() {
//         let matrix: Vec<Vec<u8>> = vec![
//             vec![0, 0, 0, 0],
//             vec![0, 0, 1, 1],
//             vec![0, 1, 0, 1],
//             vec![0, 1, 1, 0],
//         ];

//         let matrix = graph::AdjacencyMatrix::from(matrix);
//         let mut excluded: Vec<Vec<usize>> = Vec::new();

//         loop {
//             let cycle = find_ham_cycle_from_vertex(matrix.clone(), 1, &excluded);
//             match cycle {
//                 Ok(val) => {
//                     println!("{:?}", val);
//                     excluded.push(val);
//                 }
//                 Err(_) => break,
//             }
//         }
//     }

//     #[test]
//     fn test_no_ham_cycle() {
//         let matrix: Vec<Vec<u8>> = vec![
//             vec![0, 0, 0, 0],
//             vec![0, 0, 1, 0],
//             vec![0, 1, 0, 1],
//             vec![0, 0, 1, 0],
//         ];

//         let matrix = graph::AdjacencyMatrix::from(matrix);

//         let cycle = find_ham_cycle_from_vertex(matrix, 1, &vec![]);

//         assert_eq!(cycle, Err(()));
//     }
// }
