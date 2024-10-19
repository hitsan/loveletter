use crate::card::card::Card;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, PartialEq)]
pub struct Deck(Vec<Card>);

impl Deck {
    fn new(cards: Vec<Card>) -> Self {
        Deck(cards) 
    }

    fn shuffle(mut cards: Vec<Card>) -> Vec<Card> {
        let mut rng = thread_rng();
        cards.shuffle(&mut rng);
        cards
    }

    pub fn init(cards: Vec<Card>, minimum_card_num: usize) -> Result<Self, String> {
        if cards.len() < minimum_card_num {
            let quation = format!("A minimum of {} cards is required.", minimum_card_num);
            return Err(quation)
        }
        let cards = Self::shuffle(cards);
        let deck = Self::new(cards);
        Ok(deck)
    }

    pub fn draw(self) -> Option<(Card, Self)> {
        match self {
            Self(mut cards) => {
                let card = cards.pop()?;
                let deck = Self::new(cards);
                Some((card, deck))
            }
        }
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
        assert_eq!(deck, case);
    }

    #[test]
    fn test_draw() {
        let input = vec![
            Card::Princess,
            Card::Minister,
            ];
        let deck = Deck::new(input);
        let (card, deck) = deck.draw().unwrap();
        assert_eq!(card, Card::Minister);
        assert_eq!(deck, Deck::new(vec![Card::Princess]));

        let (card, deck) = deck.draw().unwrap();
        assert_eq!(card, Card::Princess);
        assert_eq!(deck, Deck::new(vec![]));

        let result = deck.draw();
        assert_eq!(result, None);
    }
}