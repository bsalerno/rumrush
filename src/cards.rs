use rand::prelude::*;
use rand::thread_rng;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let p = match *self {
            Suit::Clubs => "♣",
            Suit::Diamonds => "♦",
            Suit::Hearts => "♥",
            Suit::Spades => "♠",
        };

        write!(f, "{}", p)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let p = match *self {
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "T",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
        };

        write!(f, "{}", p)
    }
}

pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        Hand { cards: Vec::new() }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn score(&self) -> i32 {
        let mut score = 0;
        for card in &self.cards {
            match card.rank {
                Rank::Ace => {
                    score += 1;
                }
                Rank::Two => {
                    score += 2;
                }
                Rank::Three => {
                    score += 3;
                }
                Rank::Four => {
                    score += 4;
                }
                Rank::Five => {
                    score += 5;
                }
                Rank::Six => {
                    score += 6;
                }
                Rank::Seven => {
                    score += 7;
                }
                Rank::Eight => {
                    score += 8;
                }
                Rank::Nine => {
                    score += 9;
                }
                Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => {
                    score += 10;
                }
            }
        }

        score
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for item in self.cards.iter() {
            write!(f, "{} ", item)?
        }
        Ok(())
    }
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);
        let suits = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
        let ranks = [
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ];

        // iterate through suits and ranks to create deck
        for &s in &suits {
            for &r in &ranks {
                cards.push(Card { suit: s, rank: r });
            }
        }

        //using shorthand initialization
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_deck_has_52_cards() {
        assert_eq!(Deck::new().cards.len(), 52);
    }

    #[test]
    fn new_deck_has_correct_cards() {
        let deck: Deck = Deck::new();

        // suits
        let mut clubs = 0;
        let mut diamonds = 0;
        let mut hearts = 0;
        let mut spades = 0;

        // ranks
        let mut aces = 0;
        let mut twos = 0;
        let mut threes = 0;
        let mut fours = 0;
        let mut fives = 0;
        let mut sixes = 0;
        let mut sevens = 0;
        let mut eights = 0;
        let mut nines = 0;
        let mut tens = 0;
        let mut jacks = 0;
        let mut queens = 0;
        let mut kings = 0;

        for i in deck.cards {
            match i.suit {
                Suit::Clubs => clubs += 1,
                Suit::Diamonds => diamonds += 1,
                Suit::Hearts => hearts += 1,
                Suit::Spades => spades += 1,
            };
            match i.rank {
                Rank::Ace => aces += 1,
                Rank::Two => twos += 1,
                Rank::Three => threes += 1,
                Rank::Four => fours += 1,
                Rank::Five => fives += 1,
                Rank::Six => sixes += 1,
                Rank::Seven => sevens += 1,
                Rank::Eight => eights += 1,
                Rank::Nine => nines += 1,
                Rank::Ten => tens += 1,
                Rank::Jack => jacks += 1,
                Rank::Queen => queens += 1,
                Rank::King => kings += 1,
            };
        }

        let exp_suit_in_deck: usize = 13;
        let exp_rank_in_deck: usize = 4;
        // suits
        assert_eq!(clubs, exp_suit_in_deck);
        assert_eq!(diamonds, exp_suit_in_deck);
        assert_eq!(hearts, exp_suit_in_deck);
        assert_eq!(spades, exp_suit_in_deck);
        // ranks
        assert_eq!(aces, exp_rank_in_deck);
        assert_eq!(twos, exp_rank_in_deck);
        assert_eq!(threes, exp_rank_in_deck);
        assert_eq!(fours, exp_rank_in_deck);
        assert_eq!(fives, exp_rank_in_deck);
        assert_eq!(sixes, exp_rank_in_deck);
        assert_eq!(sevens, exp_rank_in_deck);
        assert_eq!(eights, exp_rank_in_deck);
        assert_eq!(nines, exp_rank_in_deck);
        assert_eq!(tens, exp_rank_in_deck);
        assert_eq!(jacks, exp_rank_in_deck);
        assert_eq!(queens, exp_rank_in_deck);
        assert_eq!(kings, exp_rank_in_deck);
    }
}
