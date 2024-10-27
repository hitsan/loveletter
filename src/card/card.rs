pub type Strength = u8;

#[derive(Debug, PartialEq)]
pub enum Class {
    Princess(Strength),
    Minister(Strength),
    General(Strength),
    Magician(Strength),
    Monk(Strength),
    Knight(Strength),
    Clown(Strength),
    Soldier(Strength),
}

#[derive(Debug, PartialEq)]
pub struct Card {
    class: Class,
    effect: Effect,
}

impl Card {
    fn new(class: Class, effect: Effect) -> Self {
        Self{class, effect}
    }
    pub  fn princess() -> Self {
        Self::new(Class::Princess(8), Effect::Discarded)
    }
    pub  fn minister() -> Self {
        Self::new(Class::Minister(7), Effect::Having)
    }
    pub  fn general() -> Self {
        Self::new(Class::General(6), Effect::Using)
    }
    pub  fn magician() -> Self {
        Self::new(Class::Magician(5), Effect::Using)
    }
    pub  fn monk() -> Self {
        Self::new(Class::Monk(4), Effect::Wating)
    }
    pub  fn knight() -> Self {
        Self::new(Class::Knight(3), Effect::Using)
    }
    pub  fn clown() -> Self {
        Self::new(Class::Clown(2), Effect::Using)
    }
    pub  fn soldier() -> Self {
        Self::new(Class::Soldier(1), Effect::Using)
    }
}
#[derive(Debug, PartialEq)]
enum Effect {
    Using,
    Discarded,
    Wating,
    Having,
}

pub fn init_card() -> Vec<Card> {
    let princess = Card::princess();
    let minister = Card::minister();
    let general = Card::general();
    let magician = Card::magician();
    let monk = Card::monk();
    let knight = Card::knight();
    let clown = Card::clown();
    let soldier = Card::soldier();
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