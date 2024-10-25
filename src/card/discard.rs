use crate::card::card::Card;
use crate::game::board::Name;

#[derive(Debug, PartialEq)]
pub struct Discard {
    name: Name,
    card: Card
}

impl Discard {
    pub fn new(name: Name, card: Card) -> Self {
        Self { name, card }
    }
}

#[derive(Debug, PartialEq)]
pub struct DiscardPile(Vec<Discard>);

impl DiscardPile {
    pub fn new() -> Self {
        let no_cards: Vec<Discard> = vec![];
        Self(no_cards)
    }
    pub fn added(self, discard: Discard) -> Self {
        match self {
            Self(mut discards) => {
                discards.push(discard);
                Self(discards)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discard() {
        let discard = Discard::new("p1".to_string(), Card::create("soldier"));
        assert_eq!(discard, Discard{ name: "p1".to_string(), card: Card::create("soldier") });

        let discard = Discard::new("p2".to_string(), Card::create("clown"));
        assert_eq!(discard, Discard{ name: "p2".to_string(), card: Card::create("clown") });
    }

    #[test]
    fn test_discard_pile() {
        let discard_pile = DiscardPile::new();
        assert_eq!(discard_pile, DiscardPile(vec![]));

        let discard_pile = DiscardPile::new();
        let discard1 = Discard::new("p1".to_string(), Card::create("soldier"));
        let discard_pile = discard_pile.added(discard1);
        let discard1 = Discard::new("p1".to_string(), Card::create("soldier"));
        assert_eq!(discard_pile, DiscardPile(vec![discard1]));

        let discard2 = Discard::new("p2".to_string(), Card::create("clown"));
        let discard_pile = discard_pile.added(discard2);
        let discard1 = Discard::new("p1".to_string(), Card::create("soldier"));
        let discard2 = Discard::new("p2".to_string(), Card::create("clown"));
        assert_eq!(discard_pile, DiscardPile(vec![discard1, discard2]));
    }

    #[test]
    fn test_added_discard_pile() {
        let discard_pile = DiscardPile::new();
        let discard = Discard::new("p1".to_string(), Card::create("soldier"));
        let discard_pile = discard_pile.added(discard);
        let discard = Discard::new("p1".to_string(), Card::create("soldier"));
        assert_eq!(discard_pile, DiscardPile(vec![discard]));

        let discard = Discard::new("p2".to_string(), Card::create("monk"));
        let discard_pile = discard_pile.added(discard);

        let discard1 = Discard::new("p1".to_string(), Card::create("soldier"));
        let discard2 = Discard::new("p2".to_string(), Card::create("monk"));
        assert_eq!(discard_pile, DiscardPile(vec![discard1, discard2]));
    }
}