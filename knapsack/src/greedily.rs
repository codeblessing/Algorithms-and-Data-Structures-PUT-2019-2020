use crate::object::*;
use std::cmp::Ordering;

#[derive(Debug)]
struct RatedObject {
    weight: usize,
    value: usize,
    ratio: f32,
    name: String,
    id: usize,
}

impl From<&Object> for RatedObject {
    fn from(object: &Object) -> Self {
        Self {
            weight: object.weight,
            value: object.value,
            ratio: object.value as f32 / object.weight as f32,
            name: object.name.clone(),
            id: object.id
        }
    }
}

pub fn pack_a_ruck(knapsack: Knapsack, objects: &Vec<Object>) -> Vec<Object> {
    let mut objects: Vec<RatedObject> = objects
        .iter()
        .map(|object| RatedObject::from(object))
        .collect();

    objects.sort_by(
        |x, y| match x.ratio.partial_cmp(&y.ratio).unwrap() {
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less,
            Ordering::Equal => Ordering::Equal,
        },
    );

    dbg!(&objects);

    let mut current_weight: usize = 0;
    let mut current_value: usize = 0;
    let mut packed: Vec<Object> = Vec::new();

    while !objects.is_empty() {
        dbg!(current_weight);
        dbg!(current_value);
        dbg!(objects[0].weight);
        if current_weight + objects[0].weight <= knapsack.capacity {
            let object = objects.remove(0);
            current_weight += object.weight;
            current_value += object.value;
            packed.push(Object::from(object.id, &object.name, object.weight, object.value));
        } else {
            objects.remove(0);
        }
    }

    packed
}


#[cfg(test)]
mod test_greedy {
    use super::*;

    #[test]
    fn test_find_solution() {
        let mut objects: Vec<Object> = Vec::new();
        objects.push(Object::from(1, "", 2, 6));
        objects.push(Object::from(2, "", 4, 4));
        objects.push(Object::from(3, "", 1, 3));
        objects.push(Object::from(4, "", 2, 4));
        objects.push(Object::from(5, "", 1, 5));
        let sack = Knapsack { capacity: 8 };
        
        let solution = pack_a_ruck(sack, &objects);

        let ids: Vec<usize> = solution.iter().map(|obj| obj.id).collect();
        let weight: usize = solution.iter().map(|obj| obj.weight).sum();
        let value: usize = solution.iter().map(|obj| obj.value).sum();

        assert_eq!(ids, vec![5, 1, 3, 4]);
        assert_eq!(weight, 6);
        assert_eq!(value, 18);
    }

    #[test]
    fn test_no_solution() {
        let mut objects: Vec<Object> = Vec::new();
        objects.push(Object::from(1, "", 9,  6));
        objects.push(Object::from(2, "", 13, 4));
        objects.push(Object::from(3, "", 22, 3));
        objects.push(Object::from(4, "", 10, 4));
        objects.push(Object::from(5, "", 15, 5));
        let sack = Knapsack { capacity: 8 };
        
        let solution = pack_a_ruck(sack, &objects);

        assert_eq!(solution, vec![]);
    }
}