// use crate::graph::AdjacencyMatrix;

// pub fn euler_cycle(mut graph: AdjacencyMatrix) -> Result<Vec<usize>, ()> {
//     let size = graph.matrix().len();

//     if (1..size).any(|vert| graph.deg(vert) % 2 == 1) {
//         println!("Graf wejściowy jest acykliczny (Nie zawiera cyklu Eulera).");
//         return Err(());
//     }

//     let mut cycle: Vec<usize> = Vec::new();
//     let vertex: usize = 1;

//     find_euler_cycle(vertex, &mut graph, &mut cycle);

//     if graph.has_edges() {
//         println!("Graf wejściowy jest acykliczny (Nie zawiera cyklu Eulera).");
//         return Err(());
//     }

//     Ok(cycle)
// }

// fn find_euler_cycle(vertex: usize, matrix: &mut AdjacencyMatrix, stack: &mut Vec<usize>) {
//     loop {
//         match matrix.next(vertex) {
//             None => break,
//             Some(next) => {
//                 matrix.remove_edge(vertex, next).unwrap_or(());
//                 find_euler_cycle(next, matrix, stack);
//             }
//         }
//     }
//     stack.push(vertex);
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

//         let cycle = euler_cycle(matrix);

//         assert!(cycle.is_ok());

//         eprintln!("{:?}", cycle.unwrap());
//     }

//     #[test]
//     fn test_no_euler_cycle() {
//         let matrix: Vec<Vec<u8>> = vec![
//             vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
//             vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
//             vec![0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
//             vec![0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0],
//             vec![0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0],
//             vec![0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0],
//             vec![0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0],
//             vec![0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0],
//             vec![0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0],
//             vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0],
//             vec![0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0],
//             vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
//             vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
//             vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0],
//         ];

//         let matrix = AdjacencyMatrix::from(matrix);

//         let cycle = euler_cycle(matrix);

//         assert!(cycle.is_err());
//     }
// }
