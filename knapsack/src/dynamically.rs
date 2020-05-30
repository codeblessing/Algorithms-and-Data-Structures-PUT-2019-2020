use crate::object::*;
use std::cmp;

struct Array2D {
    columns: usize,
    rows: usize,
    array: Vec<Vec<usize>>
}

pub fn pack_a_ruck(knapsack: Knapsack, objects: &Vec<Object>) -> Vec<Object> {
    let matrix = create_dynamic_matrix(knapsack, objects);
    packed_objects(&matrix, objects)
}

fn create_dynamic_matrix(knapsack: Knapsack, objects: &Vec<Object>) -> Array2D {
    let rows = objects.len() + 1;
    let columns = knapsack.capacity + 1;
    let mut matrix: Vec<Vec<usize>> = vec![vec![0; columns]; rows];
    
    for row in 1..rows {
        for column in 1..columns {
            let last_optimal = matrix[row - 1][column];
            let object = &objects[row - 1];
            
            if object.weight > column {
                matrix[row][column] = last_optimal;
            } else {
                matrix[row][column] = cmp::max(last_optimal, matrix[row - 1][column - object.weight] + object.value);
            }
        }
    }
    
    Array2D { rows, columns, array: matrix }
}

fn packed_objects(array: &Array2D, objects: &Vec<Object>) -> Vec<Object> {
    let mut row = array.rows - 1;
    let mut column = array.columns - 1;
    let matrix = &array.array;

    let mut object: Object;
    let mut packed: Vec<Object> = Vec::new();

    loop {
        if matrix[row][column] == matrix[row - 1][column] {
            row -= 1;
        } else {
            object = objects[row - 1].clone();
            row -= 1;
            column -= object.weight;
            packed.push(object);
        }
        if matrix[row][column] == 0 {
            break;
        }
    }

    packed
}

#[cfg(test)]
mod test_dynamic {
    use super::*;

    #[test]
    fn test_create_dynamic_matrix() {
        let knapsack = Knapsack { capacity: 8 };
        let mut objects: Vec<Object> = Vec::new();
        objects.push(Object::from(1, "", 2, 4));
        objects.push(Object::from(2, "", 1, 3));
        objects.push(Object::from(3, "", 4, 6));
        objects.push(Object::from(4, "", 4, 8));

        let dynamic_matrix: Vec<Vec<usize>> = vec![
            vec![0, 0, 0, 0, 0,  0,  0,  0,  0],
            vec![0, 0, 4, 4, 4,  4,  4,  4,  4],
            vec![0, 3, 4, 7, 7,  7,  7,  7,  7],
            vec![0, 3, 4, 7, 7,  9, 10, 13, 13],
            vec![0, 3, 4, 7, 8, 11, 12, 15, 15]
        ];

        let generated_matrix = create_dynamic_matrix(knapsack, &objects);

        assert_eq!(dynamic_matrix, generated_matrix.array);
    }

    #[test]
    fn test_pack_a_ruck() {
        let knapsack = Knapsack { capacity: 8 };
        let mut objects: Vec<Object> = Vec::new();
        objects.push(Object::from(1, "", 2, 4));
        objects.push(Object::from(2, "", 1, 3));
        objects.push(Object::from(3, "", 4, 6));
        objects.push(Object::from(4, "", 4, 8));

        let packed = pack_a_ruck(knapsack, &objects);

        let weight: usize = packed.iter().map(|obj| obj.weight).sum();
        let value: usize = packed.iter().map(|obj| obj.value).sum();
        let ids: Vec<usize> = packed.iter().map(|obj| obj.id).collect();

        assert_eq!(weight, 7);
        assert_eq!(value, 15);
        assert_eq!(ids, vec![4, 2, 1]);
    }

    #[test]
    fn test_no_solution() {
        let knapsack = Knapsack { capacity: 8 };
        let mut objects: Vec<Object> = Vec::new();
        objects.push(Object::from(1, "", 9, 4));
        objects.push(Object::from(2, "", 11, 3));
        objects.push(Object::from(3, "", 42, 6));
        objects.push(Object::from(4, "", 24, 8));

        let packed = pack_a_ruck(knapsack, &objects);

        assert_eq!(packed, Vec::new());
    }
}