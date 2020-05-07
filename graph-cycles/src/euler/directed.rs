// use crate::graph::SuccessorsList;
// use std::{cmp::Ordering, collections::HashMap};

// fn euler_cycle(graph: SuccessorsList) -> Result<Vec<usize>, ()> {
//     let mut list = graph.list();
//     let size = list.len();

//     if (1..size).any(|vert| deg(vert, &list) == 1) {
//         println!("Graf wejściowy jest acykliczny (Nie zawiera cyklu Eulera).");
//         return Err(());
//     }

//     let mut cycle: Vec<usize> = Vec::new();
//     let vertex: usize = 1;

//     find_euler_cycle(vertex, &mut list, &mut cycle);

//     Ok(cycle)
// }

// fn find_euler_cycle(vertex: usize, list: &mut HashMap<usize, Vec<usize>>, stack: &mut Vec<usize>) {
//     loop {
//         match next(vertex, list) {
//             None => break,
//             Some(next) => {
//                 remove_edge(vertex, next, list);
//                 find_euler_cycle(next, list, stack);
//             }
//         }
//     }
//     stack.push(vertex);
// }

// fn remove_edge(first: usize, second: usize, list: &mut HashMap<usize, Vec<usize>>) {
//     list[&first] = list[&first].iter().filter(|&&vert| vert != second).copied().collect();
// }

// fn next(vertex: usize, matrix: &HashMap<usize, Vec<usize>>) -> Option<usize> {
//     let successors: Vec<usize> = matrix[vertex]
//         .iter()
//         .enumerate()
//         .filter(|&(_, &val)| val == 1)
//         .map(|(key, _)| key)
//         .collect();
//     if successors.is_empty() {
//         None
//     } else {
//         Some(successors[0])
//     }
// }

// fn has_edges(matrix: &Vec<Vec<u8>>) -> bool {
//     matrix.iter().any(|row| row.iter().any(|&val| val != 0))
// }

// fn deg(vertex: usize, list: &HashMap<usize, Vec<usize>>) -> usize {
//     let predecessors_count: usize = 0;
//     list.iter().for_each(|(_, succ)| {
//         if succ.contains(&vertex) {
//             predecessors_count += 1;
//         }
//     });

//     match list[&vertex].len().cmp(&predecessors_count) {
//         Ordering::Less | Ordering::Greater => 1,
//         Ordering::Equal => 0,
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::graph::AdjacencyMatrix;

//     #[test]
//     fn test_find_euler_cycle() {
//         let matrix: Vec<Vec<u8>> = vec![
//             vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
//             vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1],
//             vec![0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0],
//             vec![0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1],
//             vec![0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0],
//             vec![0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0],
//             vec![0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0],
//             vec![0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1],
//             vec![0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0],
//             vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
//             vec![0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0],
//         ];

//         let matrix = AdjacencyMatrix::from(matrix);

//         let cycle = euler_cycle(matrix).unwrap();

//         eprintln!("{:?}", cycle);
//     }
// }
