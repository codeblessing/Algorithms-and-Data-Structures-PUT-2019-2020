use std::str::FromStr;
use failure::{Fail, Error};
use regex::Regex;
use lazy_static::lazy_static;

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
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static!{
            static ref SIMPLE: Regex = Regex::new(r"^\s*(?P<weight>\d+)\s+(?P<value>\d+)\s*$").unwrap();
            static ref FULL: Regex = Regex::new(r"^\s*(?P<id>\d*)\s*(?P<name>[a-zA-ZĄĆĘŁŃÓŚŹŻąćęłńóśźż\s]*)\s*(?P<weight>\d+)\s+(?P<value>\d+)\s*$").unwrap();
        }

        let simple_state = SIMPLE.captures(s);
        if let Some(captures) = simple_state {
            let id = unsafe { OBJECT_ID };
            let name = "".to_owned();
            let weight = captures["weight"].parse()?;
            let value = captures["value"].parse()?;
            unsafe { OBJECT_ID += 1; }
            return Ok(Self { id, name, weight, value });
        } else {
            let captures = FULL.captures(s).ok_or(ObjectParseError{})?;
            let id = if &captures["id"] == "" { unsafe { OBJECT_ID } } else { captures["id"].parse()? };
            let name = captures["name"].to_owned();
            let weight = captures["weight"].parse()?;
            let value = captures["value"].parse()?;
            return Ok(Self { id, name, weight, value });
        }
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:\n\tid: {}\n\twaga: {}\n\twartość: {}", self.name, self.id, self.weight, self.value )
    }
}

#[derive(Debug, Fail)]
#[fail(display = "Nie można stworzyć przedmiotu.")]
struct ObjectParseError {}