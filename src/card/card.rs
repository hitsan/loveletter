// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct Card {
//     kind: Card,
// }

// impl Card {
//     pub fn new(kind: Card) -> Self {
//         Self{kind}
//     }

//     pub fn create(kind: &str) -> Self {
//         let kind = Card::new(kind);
//         Self::new(kind)
//     }
// }

pub type Strength = u8;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Card {
    Princess(Strength),
    Minister(Strength),
    General(Strength),
    Magician(Strength),
    Monk(Strength),
    Knight(Strength),
    Clown(Strength),
    Soldier(Strength),
}

impl Card {
    pub fn new(kind_name: &str) -> Self {
        match kind_name {
            "princess" => Card::Princess(8),
            "minister" => Card::Minister(7),
            "general" => Card::General(6),
            "magician" => Card::Magician(5),
            "monk" => Card::Monk(4),
            "knight" => Card::Knight(3),
            "clown" => Card::Clown(2),
            "soldier" => Card::Soldier(1),
            _ => panic!("Illegal card")
        }
    }
}

pub fn init_card() -> Vec<Card> {
    let princess = Card::new("princess");
    let minister = Card::new("minister");
    let general = Card::new("general");
    let magician = Card::new("magician");
    let monk = Card::new("monk");
    let knight = Card::new("knight");
    let clown = Card::new("clown");
    let soldier = Card::new("soldier");
    vec![
        princess,
        minister,
        general,
        magician,
        monk,
        knight,
        clown,
        soldier,
    ]
}