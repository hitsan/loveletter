use crate::user::player::Player;
use crate::user::looser::Looser;
use crate::card::card::Card;
use crate::card::deck::Deck;
use crate::card::discard::DiscardPile;
pub type Name = String;

#[derive(Debug, PartialEq)]
struct Board {
    players: Vec<Player>,
    loosers: Vec<Looser>,
    deck: Deck,
    discard_pile: DiscardPile,
    sercret_card: Card
}

impl Board {
    pub fn new(players: Vec<Player>, loosers: Vec<Looser>, deck: Deck, discard_pile: DiscardPile, sercret_card: Card) -> Self {
        Self {
            players,
            loosers,
            deck,
            discard_pile,
            sercret_card
        }
    }

    pub fn next_player(mut self) -> (Player, Self) {
        let player = self.players.pop().unwrap();
        let board = Self::new(self.players, self.loosers, self.deck, self.discard_pile, self.sercret_card);
        (player, board)
    }

    pub fn added(mut self, player: Player) -> Self {
        self.players.insert(0, player);
        Self::new(self.players, self.loosers, self.deck, self.discard_pile, self.sercret_card)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() -> Board {
        let player1 = Player::new("p1".to_string(), Card::Clown);
        let player2 = Player::new("p2".to_string(), Card::Soldier);
        let player3 = Player::new("p3".to_string(), Card::Monk);
        let players = vec![player1, player2, player3];
        let loosers: Vec<Looser> = vec![];
        let deck = Deck::new(vec![
            Card::Magician,
            Card::Princess,
            Card::Knight,
        ]);
        let discard_pile = DiscardPile::new();
        let sercret_card = Card::Minister;
        Board::new(players, loosers, deck, discard_pile, sercret_card)
    }

    #[test]
    fn test_add_finish_player() {
        let board = init();
        let finished_player = Player::new("p4".to_string(), Card::Magician);
        let board = board.added(finished_player);
        fn test_data() -> Board {
            let finished_player = Player::new("p4".to_string(), Card::Magician);
            let player1 = Player::new("p1".to_string(), Card::Clown);
            let player2 = Player::new("p2".to_string(), Card::Soldier);
            let player3 = Player::new("p3".to_string(), Card::Monk);
            let players = vec![finished_player, player1, player2, player3];
            let loosers: Vec<Looser> = vec![];
            let deck = Deck::new(vec![
                Card::Magician,
                Card::Princess,
                Card::Knight,
            ]);
            let discard_pile = DiscardPile::new();
            let sercret_card = Card::Minister;
            Board::new(players, loosers, deck, discard_pile, sercret_card)
        }
        assert_eq!(board, test_data());
    }
}