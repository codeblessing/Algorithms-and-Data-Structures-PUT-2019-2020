//! Topological sorting using zero-in node removal technique and graph matrix graph representation.

use crate::graph;

/// Sorts graph using zero-in node removal technique and adjacency matrix graph representation.
/// When graph contains cycle returns empty `Vec` otherwise returns `Vec` with sorted nodes.
pub fn sort(matrix: &graph::Matrix) -> Vec<usize> {
    let size = matrix.vertex_count();

    if size == 0 {
        return vec![];
    }
    
    // predecessors column
    let p = size + 2;

    let mut matrix = matrix.matrix();
    matrix[0][p] = -1;
    
    let mut sorted: Vec<usize> = Vec::new();

    let mut vertex: usize;

    loop {
        match matrix.iter_mut().enumerate().find(|(_, row)| row[p] == 0) {
            Some((key, row)) => {
                vertex = key;
                row[p] = -1
            },
            None => {
                println!("Cannot sort graph with cycles.");
                sorted = vec![];
                break;
            }
        };

        eprintln!("{}", pretty_matrix(&matrix));

        matrix.iter_mut().for_each(|row| {
            let next_pred = if row[vertex].abs() as usize % (size + 1) == vertex {
                0
            } else {
                row[vertex] as usize % (size + 1)
            };

            eprintln!("Vertex: {}", vertex);
            eprintln!("Next predecessor: {}", next_pred);
            eprintln!("Row: {:?}", row);

            for (key, &val) in row.clone().iter().enumerate() {
                if val != -1 && val.abs() as usize  % (size + 1) == vertex {
                    row[key] = next_pred as isize;
                }
            }
        });

        sorted.push(vertex);

        if matrix.iter().all(|row| row[p] == -1) {
            break;
        }
    }

    sorted
}

fn pretty_matrix(matrix: &[Vec<isize>]) -> String {
    let mut out: String = String::new();
    for line in matrix {
        out += line
            .iter()
            .map(|val| format!("{:^3}", val))
            .fold(String::new(), |acc, val| format!("{}{}", acc, val))
            .chars()
            .chain(std::iter::once('\n'))
            .collect::<Vec<char>>()
            .iter()
            .cloned()
            .collect::<String>()
            .as_str();
    }
    out
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::graph;

    #[test]
    fn test_sort_graph_matrix() {
        let graph: Vec<(usize, usize)> = vec![(1, 2), (1, 5), (1, 3), (2, 5), (3, 2), (3, 4), (4, 2), (4, 5)];
        let matrix = graph::Matrix::from(&graph, 5);
        let sorted = sort(&matrix);
        assert_eq!(sorted, vec![1, 3, 4, 2, 5])
    }

    #[test]
    fn test_sort_with_cycle() {
        let graph: Vec<(usize, usize)> =
            vec![(1, 5), (1, 3), (2, 1), (2, 5), (3, 2), (3, 4), (4, 5)];
        let matrix = graph::Matrix::from(&graph, 5);
        let sorted = sort(&matrix);
        assert_eq!(sorted, vec![])
    }
}