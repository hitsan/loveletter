use crate::card::card::Card;
use rand::seq::SliceRandom;
use rand::{thread_rng, Error};

#[derive(Debug, PartialEq)]
pub struct Deck(Vec<Card>);

impl Deck {
    pub fn new(mut cards: Vec<Card>) -> Self {
        if cards.len() < 16 {
            panic!("Illigal number of card!");
        }
        let mut rng = thread_rng();
        cards.shuffle(&mut rng);
        Deck(cards) 
    }

    pub fn pick(&mut self, num: usize) -> Vec<Card> {
        let mut ret: Vec<Card> = vec![];
        for _ in 0..num {
            let card = if let Some(card) = self.next() {
                card
            } else {
                panic!("Cannot start")
            };
            ret.push(card);
        }
        ret
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
            Card::Monk,
            Card::Knight,
            Card::Clown,
            Card::Soldier,
            Card::Monk,
            Card::Knight,
            Card::Clown,
            Card::Soldier
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