use crate::card::card::Card;
use crate::card::discard::Discard;
use crate::game::board::Name;

#[derive(Debug, PartialEq)]
pub struct Player{
    name: Name,
    card: Card,
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
}

#[derive(Debug, PartialEq)]
pub struct Looser {
    name: Name
}

impl Looser {
    pub fn new(name: Name) -> Self {
        Self{name}
    }
}

#[derive(Debug, PartialEq)]
pub struct Players(Vec<Player>);

impl Players {
    pub fn next(mut self) -> Option<(Player, Self)> {
        match self {
            Self(mut players) => {
                let player = players.pop()?;
                let players = Self(players);
                Some((player, players))
            }
        }
    }

    pub fn peek(&self, target: &Name) -> Option<&Player> {
        self.0.iter().find(|p| p.name == *target)
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

    fn players_data() -> Players {
        let player1 = Player::new("p1".to_string(), Card::Clown);
        let player2 = Player::new("p2".to_string(), Card::Soldier);
        let player3 = Player::new("p3".to_string(), Card::Monk);
        let players = vec![player1, player2, player3];
        Players(players)
    }

    #[test]
    fn test_next_player() {
        let players = players_data();
        let (player, players) = players.next().unwrap();
        let name = "p3".to_string();
        assert_eq!(player, Player::new(name, Card::Monk));
        let expect_players = || {
            let player1 = Player::new("p1".to_string(), Card::Clown);
            let player2 = Player::new("p2".to_string(), Card::Soldier);
            let players = vec![player1, player2];
            Players(players)
        };
        assert_eq!(players, expect_players());

        let (player, players) = players.next().unwrap();
        let name = "p2".to_string();
        assert_eq!(player, Player::new(name, Card::Soldier));
        let expect_players = || {
            let player1 = Player::new("p1".to_string(), Card::Clown);
            let players = vec![player1];
            Players(players)
        };
        assert_eq!(players, expect_players());

        let (player, players) = players.next().unwrap();
        let name = "p1".to_string();
        assert_eq!(player, Player::new(name, Card::Clown));
        let expect_players = || {
            let players = vec![];
            Players(players)
        };
        assert_eq!(players, expect_players());

        let result = players.next();
        assert_eq!(result, None);
    }

    #[test]
    fn test_peek_player() {
        let players = players_data();
        let player = players.peek(&"p2".to_string()).unwrap();
        let p2 = Player::new("p2".to_string(), Card::Soldier);
        assert_eq!(player, &p2);

        let player = players.peek(&"p7".to_string());
        assert_eq!(player, None);
    }
}