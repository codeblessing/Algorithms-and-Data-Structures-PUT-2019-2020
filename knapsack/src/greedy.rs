use crate::object::*;
use std::cmp::Ordering;

struct RatedObject {
    weight: usize,
    value: usize,
    ratio: f32,
}

impl From<Object> for RatedObject {
    fn from(object: Object) -> Self {
        Self {
            weight: object.weight,
            value: object.value,
            ratio: object.value as f32 / object.weight as f32,
        }
    }
}

pub fn knapsack(sack: Knapsack, objects: Vec<Object>) -> (Vec<usize>, usize, usize) {
    let mut objects: Vec<(usize, RatedObject)> = objects
        .iter()
        .enumerate()
        .map(|(index, &object)| (index, RatedObject::from(object)))
        .collect();

    objects.sort_by(
        |(_, x), (_, y)| match x.ratio.partial_cmp(&y.ratio).unwrap() {
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less,
            Ordering::Equal => Ordering::Equal,
        },
    );

    let mut current_weight: usize = 0;
    let mut current_value: usize = 0;
    let mut positions: Vec<usize> = Vec::new();

    loop {
        if current_weight + objects[0].1.weight <= sack.capacity {
            let (index, obj) = objects.remove(0);
            positions.push(index);
            current_weight += obj.weight;
            current_value += obj.value;
        } else {
            positions.sort();
            break;
        }
    }

    (positions, current_weight, current_value)
}


#[cfg(test)]
mod test_greedy {
    use super::*;

    #[test]
    fn test_find_solution() {
        let mut objects: Vec<Object> = Vec::new();
        objects.push(Object { weight: 2, value: 6 });
        objects.push(Object { weight: 4, value: 4 });
        objects.push(Object { weight: 1, value: 3 });
        objects.push(Object { weight: 2, value: 4 });
        objects.push(Object { weight: 1, value: 5 });
        let sack = Knapsack { capacity: 8 };
        
        let solution = knapsack(sack, objects);

        assert_eq!(solution, (vec![0, 2, 3, 4], 6, 18));
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
        
        let solution = knapsack(sack, objects);

        assert_eq!(solution, (vec![], 0, 0));
    }
}