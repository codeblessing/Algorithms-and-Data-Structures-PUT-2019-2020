use std::{error::Error, str::FromStr};

static mut OBJECT_ID: usize = 1; 
#[derive(Clone, Copy)]
pub struct Knapsack {
    pub capacity: usize
}

#[derive(Clone, Debug, PartialEq)]
pub struct Object {
    pub weight: usize,
    pub value: usize,
    pub name: String,
    pub id: usize,
}

impl Object {
    pub fn from(id: usize, name: &str, weight: usize, value: usize) -> Self {
        Self {
            id,
            name: name.to_owned(),
            weight,
            value
        }
    }
}

impl FromStr for Object {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        match tokens.len() {
            2 => {
                let weight = tokens[0].parse()?;
                let value = tokens[1].parse()?;
                let id = unsafe { OBJECT_ID };
                unsafe { OBJECT_ID += 1; }
                Ok(Object {id, name: "".to_owned(), weight, value} )
            },
            3 => {
                let id = tokens[0].parse()?;
                let weight = tokens[1].parse()?;
                let value = tokens[2].parse()?;
                unsafe { OBJECT_ID = id + 1; }
                Ok(Object {id, name: "".to_owned(), weight, value} )
            }
            4 => {
                let id = tokens[0].parse()?;
                let name = tokens[1].to_owned();
                let weight = tokens[2].parse()?;
                let value = tokens[3].parse()?;
                unsafe { OBJECT_ID = id + 1; }
                Ok(Object {id, name, weight, value} )
            }
            _ => Err(Box::from(ObjectParseError{}))
        }
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
r"{}:
    id: {}
    waga: {}
    wartość: {}
", self.name, self.id, self.weight, self.value )
    }
}

#[derive(Clone, Debug)]
struct ObjectParseError {}

impl std::fmt::Display for ObjectParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot parse Object")
    }
}

impl std::error::Error for ObjectParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}