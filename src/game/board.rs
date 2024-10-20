use crate::user::player::Players;
use crate::user::looser::Looser;
use crate::card::card::Card;
use crate::card::deck::Deck;
use crate::card::discard::DiscardPile;
pub type Name = String;

#[derive(Debug, PartialEq)]
struct Board {
    players: Players,
    loosers: Vec<Looser>,
    deck: Deck,
    discard_pile: DiscardPile,
    sercret_card: Card
}

impl Board {
    pub fn new(players: Players, loosers: Vec<Looser>, deck: Deck, discard_pile: DiscardPile, sercret_card: Card) -> Self {
        Self {
            players,
            loosers,
            deck,
            discard_pile,
            sercret_card
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}