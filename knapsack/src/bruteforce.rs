use crate::object::*;
use std::sync::mpsc;

/// Returns `Vec` of positions in `objs` and maximum value.
pub fn knapsack<T>(sack: Knapsack, objs: T, thread_count: u8) -> (Vec<usize>, usize, usize)
where
    T: Into<Vec<Object>>,
{
    let objs: Vec<Object> = objs.into();
    let case_count = (2 << objs.len()) - 1;

    let (tx, rx) = mpsc::channel();
    let pool = threadpool::Builder::new()
        .num_threads(thread_count as usize)
        .build();

    for id in 1..case_count {
        let transmitter = mpsc::Sender::clone(&tx);
        let objects = objs.clone();
        pool.execute(move || {
            let out = pack(id, &objects);
            transmitter.send(out).unwrap_or(());
        })
    }

    drop(tx);

    let mut solutions: Vec<(usize, usize, usize)> = Vec::new();

    for msg in rx {
        if msg.1 <= sack.capacity {
            solutions.push(msg);
        }
    }

    let max = solutions.iter().max_by(|x, y| x.2.cmp(&y.2)).unwrap();
    let mut positions: Vec<usize> = Vec::new();
    let mut id = 1;

    for idx in 0..objs.len() {
        dbg!(idx);
        if (id & max.0) != 0 {
            positions.push(idx);
        }
        id <<= 1;
    }

    (positions, max.1, max.2)
}

/// Returns `ids`, weight and value
fn pack(ids: usize, objs: &Vec<Object>) -> (usize, usize, usize) {
    let mut id: usize = 1;
    let mut weight: usize = 0;
    let mut value: usize = 0;
    for object in objs {
        if (ids & id) != 0 {
            weight += object.weight;
            value += object.value;
        }
        id <<= 1;
    }

    dbg!(ids, weight, value)
}

#[cfg(test)]
mod test_bruteforce {
    use super::*;

    #[test]
    fn test_find_optimal() {
        let mut objects: Vec<Object> = Vec::new();
        objects.push(Object { weight: 2, value: 6 });
        objects.push(Object { weight: 4, value: 4 });
        objects.push(Object { weight: 1, value: 3 });
        objects.push(Object { weight: 2, value: 4 });
        objects.push(Object { weight: 1, value: 5 });
        let sack = Knapsack { capacity: 8 };
        
        let optimal = knapsack(sack, objects, 4);

        assert_eq!(optimal, (vec![0, 2, 3, 4], 6, 18));
    }

    #[test]
    fn test_no_solution() {
        let mut objects: Vec<Object> = Vec::new();
        objects.push(Object { weight: 9, value: 6 });
        objects.push(Object { weight: 13, value: 4 });
        objects.push(Object { weight: 22, value: 3 });
        objects.push(Object { weight: 10, value: 4 });
        objects.push(Object { weight: 15, value: 5 });
        let sack = Knapsack { capacity: 8 };
        
        let optimal = knapsack(sack, objects, 4);

        assert_eq!(optimal, (vec![], 0, 0));
    }

    #[test]
    fn test_one_solution() {
        let mut objects: Vec<Object> = Vec::new();
        objects.push(Object { weight: 8, value: 6 });
        objects.push(Object { weight: 13, value: 4 });
        objects.push(Object { weight: 22, value: 3 });
        objects.push(Object { weight: 10, value: 4 });
        objects.push(Object { weight: 15, value: 5 });
        let sack = Knapsack { capacity: 8 };
        
        let optimal = knapsack(sack, objects, 4);

        assert_eq!(optimal, (vec![0], 8, 6));
    }
}