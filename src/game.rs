use crate::cards::{Deck, Hand};
// use std::io;

pub struct Player {
    pub hand: Hand,
}

impl Player {
    pub fn new() -> Self {
        Player { hand: Hand::new() }
    }
}

pub fn play_game(deck: &mut Deck) {
    let mut player = Player::new();
    let mut dealer = Player::new();

    // deal initial (10) cards
    for _ in 1..=10 {
        player.hand.add_card(deck.deal().unwrap());
        dealer.hand.add_card(deck.deal().unwrap());
    }

    println!("Hand: {}", player.hand);
    println!("Hand: {}", dealer.hand);
}
