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

#[derive(Debug, PartialEq)]
pub struct Loosers(Vec<Looser>);

impl Loosers {
    pub fn new(loosers: Vec<Looser>) -> Loosers {
        Loosers(loosers)
    }

    pub fn added(self, looser: Looser) -> Loosers {
        let mut loosers = self.0;
        loosers.push(looser);
        Loosers(loosers)
    }
}