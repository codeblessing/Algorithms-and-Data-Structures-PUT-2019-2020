use crate::object::*;
use std::sync::mpsc;

pub fn pack_a_ruck(sack: Knapsack, objs: &Vec<Object>, thread_count: u8) -> Vec<Object> {
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

    let mut solutions = Vec::new();

    for msg in rx {
        if msg.1 <= sack.capacity {
            solutions.push(msg);
        }
    }

    let (max, ..) = solutions
        .iter()
        .max_by(|(.., x), (.., y)| x.cmp(&y))
        .unwrap()
        .clone();

    max
}

/// Returns `ids`, weight and value
fn pack(ids: usize, objs: &Vec<Object>) -> (Vec<Object>, usize, usize) {
    let mut id: usize = 1;
    let mut weight: usize = 0;
    let mut value: usize = 0;
    let mut objects: Vec<Object> = Vec::new();

    for object in objs {
        if (ids & id) != 0 {
            objects.push(object.clone());
            weight += object.weight;
            value += object.value;
        }
        id <<= 1;
    }

    (objects, weight, value)
}

#[cfg(test)]
mod test_bruteforce {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_find_optimal() {
        let mut objects: Vec<Object> = Vec::new();
        objects.push(Object::from(1, "", 2, 6));
        objects.push(Object::from(2, "", 4, 4));
        objects.push(Object::from(3, "", 1, 3));
        objects.push(Object::from(4, "", 2, 4));
        objects.push(Object::from(5, "", 1, 5));
        let sack = Knapsack { capacity: 8 };

        let optimal = pack_a_ruck(sack, &objects, 4);
        let weight: usize = optimal.iter().map(|obj| obj.weight).sum();
        let value: usize = optimal.iter().map(|obj| obj.value).sum();
        let ids: HashSet<usize> = optimal.iter().map(|obj| obj.id).collect();

        let expected_ids: HashSet<usize> = [1, 3, 4, 5].iter().copied().collect();

        assert_eq!(weight, 6);
        assert_eq!(value, 18);
        assert_eq!(ids, expected_ids);
    }

    #[test]
    fn test_no_solution() {
        let mut objects: Vec<Object> = Vec::new();
        objects.push(Object::from(1, "", 9, 6));
        objects.push(Object::from(2, "", 13, 4));
        objects.push(Object::from(3, "", 22, 3));
        objects.push(Object::from(4, "", 10, 4));
        objects.push(Object::from(5, "", 15, 5));
        let sack = Knapsack { capacity: 8 };

        let optimal = pack_a_ruck(sack, &objects, 4);

        assert_eq!(optimal, Vec::new());
    }

    #[test]
    fn test_one_solution() {
        let mut objects: Vec<Object> = Vec::new();
        objects.push(Object::from(1, "", 8, 6));
        objects.push(Object::from(2, "", 13, 4));
        objects.push(Object::from(3, "", 22, 3));
        objects.push(Object::from(4, "", 10, 4));
        objects.push(Object::from(5, "", 15, 5));
        let ruck = Knapsack { capacity: 8 };

        let optimal = pack_a_ruck(ruck, &objects, 4);
        let weight: usize = optimal.iter().map(|obj| obj.weight).sum();
        let value: usize = optimal.iter().map(|obj| obj.value).sum();
        let ids: Vec<usize> = optimal.iter().map(|obj| obj.id).collect();

        assert_eq!(weight, 8);
        assert_eq!(value, 6);
        assert_eq!(ids, vec![1]);
    }
}
