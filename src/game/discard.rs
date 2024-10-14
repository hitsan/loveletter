use crate::card::card::Card;
use crate::user::user::*;

#[derive(Debug, PartialEq)]
struct DiscardPile {
    name: String,
    cards: Vec<Card>
}

impl DiscardPile {
    fn new(name: String) -> Self {
        Self { name: name.to_string(), cards: vec![] }
    }

    fn added(&self, card: Card) -> Self {
        let mut cards = self.cards.clone();
        let name = self.name.clone();
        cards.push(card);
        Self { name, cards }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discard_pile() {
        let discard_pile = DiscardPile::new("p1".to_string());
        let discard_pile = discard_pile.added(Card::Soldier);
        assert_eq!(discard_pile, DiscardPile{ name: "p1".to_string(), cards: vec![Card::Soldier] });

        let discard_pile = discard_pile.added(Card::Clown);
        assert_eq!(discard_pile, DiscardPile{ name: "p1".to_string(), cards: vec![Card::Soldier, Card::Clown] });

        let discard_pile = discard_pile.added(Card::Monk);
        assert_eq!(discard_pile, DiscardPile{ name: "p1".to_string(), cards: vec![Card::Soldier, Card::Clown, Card::Monk] });
    }
}