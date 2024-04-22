extern crate rummy_500;
use rummy_500::game::{Player, Stockpile};

#[test]
fn test_new_deck_has_52_cards() {
    let deck = Stockpile::new();
    assert_eq!(deck.cards.len(), 52);
}

#[test]
fn test_shuffle_deck_changes_order() {
    let deck1 = Stockpile::new();
    let mut deck2 = deck1.clone();
    deck2.shuffle();
    assert_ne!(deck1.cards, deck2.cards);
}

#[test]
fn test_deal_gives_correct_number_of_cards() {
    let mut deck = Stockpile::new();
    deck.shuffle();
    let mut player = Player::new();
    let num_cards = 5;
    deck.deal(&mut player, num_cards);
    assert_eq!(player.cards.len(), num_cards);
}

#[test]
fn test_player_draws_card_from_deck() {
    let mut deck = Stockpile::new();
    deck.shuffle();
    let mut player = Player::new();
    let initial_deck_size = deck.cards.len();
    player.draw(&mut deck);
    assert_eq!(deck.cards.len(), initial_deck_size - 1);
    assert_eq!(player.cards.len(), 1);
}
