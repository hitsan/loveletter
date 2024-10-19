pub type Name = String;

// use crate::user::user::*;
// use crate::card::card::Card;
// use crate::game::discard::DiscardPile;
// use crate::game::deck::Deck;
// use rand::seq::SliceRandom;
// use rand::thread_rng;

// #[derive(Debug)]
// struct Game {
//     players: Vec<Player>,
//     deck: Deck,
//     discard_pile: DiscardPile,
//     sercret_card: Card
// }

// impl Game {
//     fn init_player(names: Vec<&str>, cards: Vec<Card>) -> Vec<WaitingPlayer> {
//         if names.len() != cards.len() {
//             panic!("illegal number")
//         }
//         let pairs = names.into_iter().zip(cards.into_iter());
//         pairs.map(
//             |(p, c)| WaitingPlayer::new(p.to_string(), c)
//         ).collect()
//     }

//     pub fn shuffle(cards: Vec<Card>) -> Vec<Card> {
//         let mut rng = thread_rng();
//         let mut cards = cards.clone();
//         cards.shuffle(&mut rng);
//         cards
//     }

//     pub fn new(player_names: Vec<&str>, cards: Vec<Card>) -> Self {
//         let mut deck = Deck::new(cards);
//         let cards = deck.pick(player_names.len());
//         let players = Self::init_player(player_names, cards);
//         let loosers: Vec<Looser> =vec![];
//         let discard_pile = DiscardPile::new();
//         let sercret_card = if let Some(card) = deck.next() {
//             card
//         } else {
//             panic!("ssss");
//         };

//         Self {
//             players,
//             loosers,
//             deck,
//             discard_pile,
//             sercret_card
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shuffle() {
    }
}