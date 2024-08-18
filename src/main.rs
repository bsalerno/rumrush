mod cards;
mod game;

use cards::Deck;
use game::play_game;

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    play_game(&mut deck);
}
