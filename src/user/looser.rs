use crate::game::board::Name;

#[derive(Debug, PartialEq)]
pub struct Looser {
    name: Name
}

impl Looser {
    pub fn new(name: Name) -> Self {
        Self{name}
    }
}