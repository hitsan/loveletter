pub type Strength = u8;

// #[derive(Debug, PartialEq, Clone, Copy)]
// pub enum Card {
//     Princess(Strength),
//     Minister(Strength),
//     General(Strength),
//     Magician(Strength),
//     Monk(Strength),
//     Knight(Strength),
//     Clown(Strength),
//     Soldier(Strength),
// }

#[derive(Debug, PartialEq)]
pub enum Card {
    Princess(Strength, Effect),
    Minister(Strength, Effect),
    General(Strength, Effect),
    Magician(Strength, Effect),
    Monk(Strength, Effect),
    Knight(Strength, Effect),
    Clown(Strength, Effect),
    Soldier(Strength, Effect),
}

impl Card {
    pub fn new(kind_name: &str) -> Self {
        match kind_name {
            "princess" => Card::Princess(8, Effect::Discarded),
            "minister" => Card::Minister(7, Effect::Having),
            "general" => Card::General(6, Effect::Using),
            "magician" => Card::Magician(5, Effect::Using),
            "monk" => Card::Monk(4, Effect::Wating),
            "knight" => Card::Knight(3, Effect::Using),
            "clown" => Card::Clown(2, Effect::Using),
            "soldier" => Card::Soldier(1, Effect::Using),
            _ => panic!("Illegal card")
        }
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