use crate::card::card::Card;
use crate::user::user::WaitingPlayer;
use crate::game::deck::Deck;

#[derive(Debug,PartialEq)]
enum WinOrLose {
    Win,
    Lose,
    Draw,
}

fn soldier_action(card: &Card, player: &WaitingPlayer) -> WinOrLose {
    if player.open() == card {
        WinOrLose::Win
    } else {
        WinOrLose::Draw
    }
}

fn clown_action(player: &WaitingPlayer) -> &Card {
    player.open()
}

fn knight_action(current: &WaitingPlayer, opponent: &WaitingPlayer) -> WinOrLose {
    let current_card = current.open();
    let opponent_card = opponent.open();
    if current_card < opponent_card {
        WinOrLose::Lose
    } else if current_card == opponent_card {
        WinOrLose::Draw
    } else {
        WinOrLose::Win
    }
}

fn magician_action(player: WaitingPlayer, mut deck: Deck) -> WaitingPlayer {
    let card = deck.next().unwrap();
    player.updated(card)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::deck::Deck;

    #[test]
    fn test_soldier() {
        let name = String::from("player1");
        let player = WaitingPlayer::new(name, Card::Princess);
        let result = soldier_action(&Card::Princess, &player);
        assert_eq!(result, WinOrLose::Win);

        let name = String::from("player1");
        let player = WaitingPlayer::new(name, Card::Princess);
        let result = soldier_action(&Card::Monk, &player);
        assert_eq!(result, WinOrLose::Draw);
    }

    #[test]
    fn test_knight() {
        let name = String::from("player1");
        let current = WaitingPlayer::new(name, Card::Princess);
        let name = String::from("player2");
        let opponent = WaitingPlayer::new(name, Card::Princess);
        let result = knight_action(&current, &opponent);
        assert_eq!(result, WinOrLose::Draw);

        let name = String::from("player1");
        let current = WaitingPlayer::new(name, Card::Monk);
        let name = String::from("player2");
        let opponent = WaitingPlayer::new(name, Card::Princess);
        let result = knight_action(&current, &opponent);
        assert_eq!(result, WinOrLose::Lose);

        let name = String::from("player1");
        let current = WaitingPlayer::new(name, Card::Princess);
        let name = String::from("player2");
        let opponent = WaitingPlayer::new(name, Card::Monk);
        let result = knight_action(&current, &opponent);
        assert_eq!(result, WinOrLose::Win);
    }

    // #[test]
    // fn test_magician() {
    //     let name = String::from("player1");
    //     let player = WaitingPlayer::new(name.clone(), Card::Princess);
    //     let input = vec![
    //         Card::Princess,
    //         Card::Minister,
    //         Card::General,
    //         Card::Magician,
    //         Card::Monk,
    //         Card::Knight,
    //         Card::Clown,
    //         Card::Soldier,
    //         Card::Monk,
    //         Card::Knight,
    //         Card::Clown,
    //         Card::Soldier,
    //         Card::Monk,
    //         Card::Knight,
    //         Card::Clown,
    //         Card::Soldier
    //         ];
    //     let deck = Deck(input);
    //     create_deck();
    //     let result = magician_action(player, deck);
    //     assert_eq!(result, WaitingPlayer::new(name, Card::Magician));
        // assert_eq!(deck, Deck::new(vec![Card::Monk]));
    // }
}