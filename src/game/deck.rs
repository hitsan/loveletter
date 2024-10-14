use crate::card::card::Card;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, PartialEq)]
pub struct Deck(Vec<Card>);

impl Deck {
    pub fn new(mut cards: Vec<Card>) -> Self {
        let mut rng = thread_rng();
        cards.shuffle(&mut rng);
        Deck(cards) 
    }
}

impl Iterator for Deck {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_deck() {
        let input = vec![
            Card::Princess,
            Card::Minister,
            Card::General,
            Card::Magician,
            Card::Monk,
            Card::Knight,
            Card::Clown,
            Card::Soldier,
            ];
        let deck = Deck::new(input.clone());
        let case = Deck(input);
        assert_ne!(deck, case);
    }

    #[test]
    fn test_next_card() {
        let input = vec![
            Card::Princess,
            Card::Minister,
            ];
        let mut deck = Deck(input);
        assert_eq!(deck.next(), Some(Card::Minister));
        assert_eq!(deck.next(), Some(Card::Princess));
        assert_eq!(deck.next(), None);
    }
}