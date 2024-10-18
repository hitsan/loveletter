use crate::{card::card::Card, game::discard::{self, Discard}};

#[derive(Debug, PartialEq)]
pub struct WaitingPlayer {
    name: String,
    card: Card
}

impl WaitingPlayer {
    pub fn new(name: String, card: Card) -> Self {
        Self{name, card}
    }

    pub fn draw(&self, card: Card) -> CurrentPlayer {
        let name = self.name.clone();
        let hand = self.card;
        CurrentPlayer::new(name, hand, card)
    }

    pub fn swap(&self, card: Card) -> (WaitingPlayer, Card) {
        let discard = self.card;
        let player = Self::new(self.name.clone(), card);
        (player, discard)
    }

    pub fn open(&self) -> &Card {
        &self.card
    }

    pub fn updated(&self, card: Card) -> Self {
        Self::new(self.name.clone(), card)
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
    pub fn new(name: String, hand: Card, drawn: Card) -> Self {
        Self{name, hand, drawn}
    }

    pub fn drop(&self, picked: Picked) -> (WaitingPlayer, Discard) {
        let name = self.name.clone();
        let (card, discard) = if picked == Picked::Hand {
            (self.drawn, self.hand)
        } else {
            (self.hand, self.drawn)
        };
        let player = WaitingPlayer::new(name.clone(), card);
        let discard = Discard::new(name.clone(), discard);
        (player, discard)
    }

    pub fn get_strength(&self) -> u32 {
        let hand = self.hand as u32;
        let drawn = self.drawn as u32;
        hand+drawn
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
    use crate::{card::card::Card, game::discard};

    #[test]
    fn test_loose() {
        let name = String::from("player1");
        let player = WaitingPlayer::new(name, Card::Princess);
        let looser = player.loose();

        let name = String::from("player1");
        let expcted = Looser::new(name);
        assert_eq!(looser, expcted);
    }

    #[test]
    fn test_draw() {
        let name = String::from("player1");
        let player = WaitingPlayer::new(name, Card::Princess);
        let player = player.draw(Card::Minister);

        let name = String::from("player1");
        assert_eq!(player, CurrentPlayer::new(name, Card::Princess, Card::Minister));
    }

    #[test]
    fn test_open() {
        let name = String::from("player1");
        let player = WaitingPlayer::new(name, Card::Princess);
        let card = player.open();
        assert_eq!(card, &Card::Princess);
        let name = String::from("player1");
        assert_eq!(player, WaitingPlayer::new(name, Card::Princess));
    }

    #[test]
    fn test_swap() {
        let name = String::from("player1");
        let player = WaitingPlayer::new(name, Card::Princess);
        let (player, card) = player.swap(Card::Monk);

        let name = String::from("player1");
        let expect = WaitingPlayer::new(name, Card::Monk);
        assert_eq!(player, expect);
        assert_eq!(card, Card::Princess);
    }

    #[test]
    fn test_drop() {
        let name = String::from("player1");
        let player = CurrentPlayer::new(name, Card::Princess, Card::Knight);
        let (player, discard) = player.drop(Picked::Hand);
        let name = String::from("player1");
        let expected = WaitingPlayer::new(name.clone(), Card::Knight);
        assert_eq!(player, expected);
        assert_eq!(discard, Discard::new(name, Card::Princess));

        let name = String::from("player1");
        let player = CurrentPlayer::new(name, Card::Princess, Card::Knight);
        let (player, discard) = player.drop(Picked::Drawn);
        let name = String::from("player1");
        let expected = WaitingPlayer::new(name.clone(), Card::Princess);
        assert_eq!(player, expected);
        assert_eq!(discard, Discard::new(name, Card::Knight));
    }

}