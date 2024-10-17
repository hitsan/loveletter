use crate::user::user::*;
use crate::card::card::Card;
use crate::game::discard::DiscardPile;
use crate::game::deck::Deck;

#[derive(Debug)]
struct Game {
    players: Vec<WaitingPlayer>,
    loosers: Vec<Looser>,
    deck: Deck,
    discard_pile: DiscardPile,
}

impl Game {
    fn init_player(names: Vec<&str>, cards: Vec<Card>) -> Vec<WaitingPlayer> {
        if names.len() != cards.len() {
            panic!("illegal number")
        }
        let pairs = names.into_iter().zip(cards.into_iter());
        pairs.map(
            |(p, c)| WaitingPlayer::new(p.to_string(), c)
        ).collect()
    }

    pub fn new(player_names: Vec<&str>, cards: Vec<Card>) -> Self {
        let mut deck = Deck::new(cards);
        let cards = deck.pick(player_names.len());
        let players = Self::init_player(player_names, cards);
        let loosers: Vec<Looser> =vec![];
        let discard_pile = DiscardPile::new();
        Self {
            players,
            loosers,
            deck,
            discard_pile
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
    }
}