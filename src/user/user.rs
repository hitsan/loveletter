use crate::card::card::Card;

#[derive(Debug, PartialEq)]
struct Player {
    name: String,
    cards: Cards
}

enum Picked {
    Hand,
    Drawn,
}

#[derive(Debug, PartialEq)]
enum Cards {
    Single(Card),
    Double{ hand: Card, drawn: Card },
}

impl Player {
    fn new(name: String, card: Card) -> Self {
        let cards = Cards::Single(card);
        Self{name, cards}
    }

    fn draw(&self, card: Card) -> Self {
        if let Cards::Single(hand) = self.cards {
            let cards = Cards::Double{ hand, drawn: card};
            let name = self.name.clone();
            return Self{name, cards}
        }
        panic!("Illegal player method")
    }

    fn open(&self, picked: Picked) -> Card {
        let (hand, drawn) = match self.cards {
            Cards::Double{hand, drawn} => (hand, drawn),
            _ => panic!("Invalid number of cards!")
        };
        match picked {
            Picked::Hand => hand,
            Picked::Drawn => drawn,
        }
    }

    fn drop(&self, picked: Picked) -> Self {
        let (hand, drawn) = match self.cards {
            Cards::Double{hand, drawn} => (hand, drawn),
            _ => panic!("Invalid number of cards!")
        };
        let name = self.name.clone();
        match picked {
            Picked::Hand => Self::new(name, drawn),
            Picked::Drawn => Self::new(name, hand),
        }
    }

    fn loose(&self) -> Looser {
        Looser::new(self.name.clone())
    }
}

#[derive(Debug, PartialEq)]
struct Looser {
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
        let player = Player::new(name, Princess);
        let looser = player.loose();

        let name = String::from("player1");
        let expcted = Looser::new(name);
        assert_eq!(looser, expcted);
    }

    #[test]
    fn test_draw() {
        let name = String::from("player1");
        let player = Player::new(name, Princess);
        let player = player.draw(Minister);

        let name = String::from("player1");
        let cards = Cards::Double {hand: Princess, drawn: Minister};
        assert_eq!(player, Player{name, cards});
    }

    #[test]
    fn test_open() {
        let name = String::from("player1");
        let cards = Cards::Double {hand: Princess, drawn: Minister};
        let player = Player { name, cards };
        let card = player.open(Picked::Hand);
        assert_eq!(card, Card::Princess);

        let name = String::from("player1");
        let cards = Cards::Double {hand: Princess, drawn: Minister};
        let player = Player { name, cards };
        let card = player.open(Picked::Drawn);
        assert_eq!(card, Card::Minister);
    }

    #[test]
    fn test_drop() {
        let name = String::from("player1");
        let cards = Cards::Double {hand: Princess, drawn: Minister};
        let player = Player { name, cards };
        let player = player.drop(Picked::Hand);
        let name = String::from("player1");
        let cards = Cards::Single(Minister);
        let expected = Player { name, cards };
        assert_eq!(player, expected);

        let name = String::from("player1");
        let cards = Cards::Double {hand: Princess, drawn: Minister};
        let player = Player { name, cards };
        let player = player.drop(Picked::Drawn);
        let name = String::from("player1");
        let cards = Cards::Single(Princess);
        let expected = Player { name, cards };
        assert_eq!(player, expected);
    }

}