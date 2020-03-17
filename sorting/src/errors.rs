use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct EmptyHeap;

impl fmt::Display for EmptyHeap {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Empty heap!")
    }
}

impl error::Error for EmptyHeap {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
