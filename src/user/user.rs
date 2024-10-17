use crate::card::card::Card;

#[derive(Debug, PartialEq)]
pub struct WaitingPlayer {
    name: String,
    card: Card
}

impl WaitingPlayer {
    pub fn new(name: String, card: Card) -> Self {
        Self{name, card}
    }

    fn draw(&self, card: Card) -> CurrentPlayer {
        let name = self.name.clone();
        let hand = self.card;
        CurrentPlayer::new(name, hand, card)
    }

    fn open(&self) -> &Card {
        &self.card
    }

    fn loose(&self) -> Looser {
        Looser::new(self.name.clone())
    }
}

#[derive(Debug, PartialEq)]
pub struct CurrentPlayer {
    name: String,
    hand: Card,
    drawn: Card,
}

impl CurrentPlayer {
    fn new(name: String, hand: Card, drawn: Card) -> Self {
        Self{name, hand, drawn}
    }

    fn drop(&self, picked: Picked) -> WaitingPlayer {
        let name = self.name.clone();
        let card = if picked == Picked::Hand { self. drawn } else { self.hand };
        WaitingPlayer::new(name, card)
    }
}

#[derive(Debug, PartialEq)]
pub enum Picked {
    Hand,
    Drawn
}

#[derive(Debug, PartialEq)]
pub struct Looser {
    name: String,
}

impl Looser {
    fn new(name: String) -> Self {
        Looser {name}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::card::Card::*;

    #[test]
    fn test_loose() {
        let name = String::from("player1");
        let player = WaitingPlayer::new(name, Princess);
        let looser = player.loose();

        let name = String::from("player1");
        let expcted = Looser::new(name);
        assert_eq!(looser, expcted);
    }

    #[test]
    fn test_draw() {
        let name = String::from("player1");
        let player = WaitingPlayer::new(name, Princess);
        let player = player.draw(Minister);

        let name = String::from("player1");
        assert_eq!(player, CurrentPlayer::new(name, Princess, Minister));
    }

    #[test]
    fn test_open() {
        let name = String::from("player1");
        let player = WaitingPlayer::new(name, Princess);
        let card = player.open();
        assert_eq!(card, &Card::Princess);
        let name = String::from("player1");
        assert_eq!(player, WaitingPlayer::new(name, Princess));
    }

    #[test]
    fn test_drop() {
        let name = String::from("player1");
        let player = CurrentPlayer::new(name, Princess, Knight);
        let player = player.drop(Picked::Hand);
        let name = String::from("player1");
        let expected = WaitingPlayer::new(name, Knight);
        assert_eq!(player, expected);

        let name = String::from("player1");
        let player = CurrentPlayer::new(name, Princess, Knight);
        let player = player.drop(Picked::Drawn);
        let name = String::from("player1");
        let expected = WaitingPlayer::new(name, Princess);
        assert_eq!(player, expected);
    }

}