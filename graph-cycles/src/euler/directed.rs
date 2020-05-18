use crate::graph::SuccessorsList;
use log::debug;

pub fn euler_cycle<T>(graph: T) -> Option<Vec<usize>>
where
    T: Into<SuccessorsList>,
{
    let mut graph = graph.into();
    let size = graph.list().len();

    if (1..size).any(|vert| graph.deg(vert) != Some(0)) || size < 3 {
        println!("Graf wejściowy jest acykliczny (Nie zawiera cyklu Eulera).");
        return None;
    }

    let mut cycle: Vec<usize> = Vec::with_capacity(size);
    let node: usize = 1;

    find_euler_cycle(node, &mut graph, &mut cycle);

    if graph.has_edges() {
        println!("Graf wejściowy jest acykliczny (Nie zawiera cyklu Eulera).");
        return None;
    }

    cycle.reverse();
    Some(cycle)
}

fn find_euler_cycle(node: usize, list: &mut SuccessorsList, stack: &mut Vec<usize>) {
    loop {
        match list.next(node) {
            None => break,
            Some(next) => {
                debug!("Next node in directed euler: {}", next);
                list.remove_edge(node, next);
                find_euler_cycle(next, list, stack);
            }
        }
    }
    stack.push(node);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::graph::SuccessorsList;
    use std::collections::HashMap;

    #[test]
    fn test_find_euler_cycle() {
        let mut list: HashMap<usize, Vec<usize>> = HashMap::new();
        list.insert(1, vec![2, 3]);
        list.insert(2, vec![3, 5]);
        list.insert(3, vec![1, 4]);
        list.insert(4, vec![2]);
        list.insert(5, vec![1]);
        let arcs = vec![(1, 2), (1, 3), (2, 3), (2, 5), (3, 1), (3, 4), (4, 2), (5, 1)];

        let list = SuccessorsList::from(arcs);

        let cycle = euler_cycle(list);

        assert!(cycle.is_some());

        eprintln!("{:?}", cycle.unwrap());
    }

    #[test]
    fn test_find_euler_cycle_from_arcs_list() {
        let arcs: Vec<(usize, usize)> = vec![
            (1, 2),
            (1, 3),
            (2, 3),
            (3, 1),
            (2, 5),
            (3, 4),
            (4, 2),
            (5, 1),
            (3, 4),
        ];

        let list = SuccessorsList::from(arcs.as_slice());

        let cycle = euler_cycle(list);

        assert!(cycle.is_some());

        eprintln!("{:?}", cycle);
    }

    #[test]
    fn test_no_euler_cycle() {
        let arcs: Vec<(usize, usize)> = vec![(1, 2), (2, 3), (3, 4), (4, 1), (5, 5)];

        let cycle = euler_cycle(arcs.as_slice());

        assert!(cycle.is_none());
    }

    #[test]
    fn test_run_with_empty() {
        let list = SuccessorsList::new();
        let cycle = euler_cycle(list);
        assert_eq!(cycle, None);
    }
}
