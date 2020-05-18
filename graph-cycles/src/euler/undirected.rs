use crate::graph::AdjacencyMatrix;

pub fn euler_cycle<T>(graph: T) -> Option<Vec<usize>>
where
    T: Into<AdjacencyMatrix>,
{
    let mut graph = graph.into();
    let size = graph.matrix().len();

    if size < 3 || (1..size).any(|vert| graph.deg(vert).unwrap() % 2 == 1) {
        println!("Graf wejściowy jest acykliczny (Nie zawiera cyklu Eulera).");
        return None;
    }

    let mut cycle: Vec<usize> = Vec::new();
    let vertex: usize = 1;

    find_euler_cycle(vertex, &mut graph, &mut cycle);

    if graph.has_edges() {
        println!("Graf wejściowy jest acykliczny (Nie zawiera cyklu Eulera).");
        return None;
    }

    Some(cycle)
}

fn find_euler_cycle(vertex: usize, matrix: &mut AdjacencyMatrix, stack: &mut Vec<usize>) {
    loop {
        match matrix.next(vertex) {
            None => break,
            Some(next) => {
                matrix.remove_edge(vertex, next);
                find_euler_cycle(next, matrix, stack);
            }
        }
    }
    stack.push(vertex);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_euler_cycle() {
        let arcs = vec![
            (10, 17),
            (1, 2),
            (1, 10),
            (2, 3),
            (3, 4),
            (3, 5),
            (3, 10),
            (4, 5),
            (4, 6),
            (4, 8),
            (5, 6),
            (5, 7),
            (6, 7),
            (6, 8),
            (7, 8),
            (7, 10),
            (8, 9),
            (9, 10),
        ];

        let cycle = euler_cycle(arcs);

        assert!(cycle.is_some());

        eprintln!("{:?}", cycle);
    }

    #[test]
    fn test_no_euler_cycle() {
        let arcs = vec![
            (13, 20),
            (1, 2),
            (1, 10),
            (2, 3),
            (3, 4),
            (3, 5),
            (3, 10),
            (4, 5),
            (4, 6),
            (4, 8),
            (5, 6),
            (5, 7),
            (6, 7),
            (6, 8),
            (7, 8),
            (7, 10),
            (8, 9),
            (9, 10),
            (11, 12),
            (11, 13),
            (12, 13),
        ];

        let cycle = euler_cycle(arcs);

        assert!(cycle.is_none());
    }

    #[test]
    fn test_run_with_empty() {
        let matrix = AdjacencyMatrix::new();
        let cycle = euler_cycle(matrix);

        assert!(cycle.is_none());
    }
}
