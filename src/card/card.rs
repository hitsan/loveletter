#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Card {
    kind: Kind,
}

impl Card {
    pub fn new(kind: Kind) -> Self {
        Self{kind}
    }

    pub fn create(kind: &str) -> Self {
        let kind = Kind::new(kind);
        Self::new(kind)
    }
}

pub type Strength = u8;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Kind {
    Princess(Strength),
    Minister(Strength),
    General(Strength),
    Magician(Strength),
    Monk(Strength),
    Knight(Strength),
    Clown(Strength),
    Soldier(Strength),
}

impl Kind {
    pub fn new(kind_name: &str) -> Self {
        match kind_name {
            "princess" => Kind::Princess(8),
            "minister" => Kind::Minister(7),
            "general" => Kind::General(6),
            "magician" => Kind::Magician(5),
            "monk" => Kind::Monk(4),
            "knight" => Kind::Knight(3),
            "clown" => Kind::Clown(2),
            "soldier" => Kind::Soldier(1),
            _ => panic!("Illegal card")
        }
    }
}

pub fn init_card() -> Vec<Card> {
    let princess = Card::new(Kind::Princess(8));
    let minister = Card::new(Kind::Minister(7));
    let general = Card::new(Kind::General(6));
    let magician = Card::new(Kind::Magician(5));
    let monk = Card::new(Kind::Monk(4));
    let knight = Card::new(Kind::Knight(3));
    let clown = Card::new(Kind::Clown(2));
    let soldier = Card::new(Kind::Soldier(1));
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