use crate::card::card::Card;
use crate::card::discard::Discard;
use crate::game::board::Name;
use crate::user::looser::Looser;

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
pub struct Players(Vec<Player>);

impl Players {
    pub fn pick_current_player(&self) -> &Player {
        let players = &self.0;
        if let Some(current_player) = players.first() {
            current_player
        } else {
            panic!("No players!");
        }
    }

    pub fn rotated(self) -> Players {
        let mut players = self.0;
        players.rotate_left(1);
        Players(players)
    }

    pub fn droped(self, name: &Name) -> Result<(Player, Players), String> {
        let mut players = self.0;
        if players.len() < 1 {
            let message = "No more players can lose!".to_string();
            return Err(message)
        }
        let index = players.iter().position(|p| p.name==*name);
        if let Some(index) = index {
            let player = players.remove(index);
            let players = Players(players);
            Ok((player, players))
        } else {
            let message = "Illgal player name!".to_string();
            Err(message)
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
    fn test_pick_current_player() {
        let players = players_data();
        let player = players.pick_current_player();
        assert_eq!(player, &Player::new("p1".to_string(), Card::Clown));
    }

    fn rotated_players_data() -> Players {
        let player1 = Player::new("p1".to_string(), Card::Clown);
        let player2 = Player::new("p2".to_string(), Card::Soldier);
        let player3 = Player::new("p3".to_string(), Card::Monk);
        let players = vec![player2, player3, player1];
        Players(players)
    }

    #[test]
    fn test_rotated_players() {
        let players = players_data();
        let players = players.rotated();
        assert_eq!(players, rotated_players_data());
    }

    fn droped_players_data() -> Players {
        let player2 = Player::new("p2".to_string(), Card::Soldier);
        let player3 = Player::new("p3".to_string(), Card::Monk);
        let players = vec![player2, player3];
        Players(players)
    }

    #[test]
    fn test_drop_player() {
        let players = players_data();
        let name = "p1".to_string();
        let (looser, players) = players.droped(&name).unwrap();
        let player = Player::new("p1".to_string(), Card::Clown);
        assert_eq!(looser, player);
        assert_eq!(players, droped_players_data());
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