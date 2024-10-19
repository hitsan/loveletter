use crate::card::card::Card;
use crate::card::discard::Discard;
use crate::game::board::Name;

#[derive(Debug, PartialEq)]
pub struct Player{
    name: Name,
    card: Card,
}

#[derive(Debug, PartialEq)]
pub struct Looser {
    name: Name
}

impl Player {
    pub fn new(name: Name, card: Card) -> Self {
        Self{name, card}
    }

    pub fn loose(self) -> (Looser, Discard) {
        match self {
            Self{name, card} => {
                let looser = Looser::new(name.clone());
                let discard = Discard::new(name.clone(), card);
                (looser, discard)
            }
        }
    }

    fn exchange(self, picked: Card) -> (Self, Card) {
        match self {
            Self{name, card} => (Self::new(name, picked), card),
        }
    }

    pub fn is(&self, name: &Name) -> bool {
        self.name == *name
    }
}

impl Looser {
    pub fn new(name: Name) -> Self {
        Self{name}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loose() {
        let name = "player".to_string();
        let player = Player::new(name.clone(), Card::Minister);
        let (looser, card) = player.loose();
        assert_eq!(looser, Looser::new(name.clone()));
        let discard = Discard::new(name.clone(), Card::Minister);
        assert_eq!(card, discard);
    }

    #[test]
    fn test_exchange() {
        let name = "player".to_string();
        let player = Player::new(name.clone(), Card::Clown);
        let (player, card) = player.exchange(Card::General);
        assert_eq!(player, Player::new(name.clone(), Card::General));
        assert_eq!(card, Card::Clown);
    }
}