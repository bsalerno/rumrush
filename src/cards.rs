use rand::prelude::*;
use rand::thread_rng;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
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

#[derive(Debug, Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

#[derive(PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn score(&self) -> i32 {
        match self.rank {
            Rank::Ace => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
        }
    }
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

    fn set_melds(&self) -> HashMap<Rank, Vec<&Card>> {
        let mut map: HashMap<Rank, Vec<&Card>> = HashMap::new();

        for c in &self.cards {
            map.entry(c.rank).or_insert_with(|| Vec::new()).push(c);
        }

        // only retain sets with 3 or more items in them
        map.retain(|_, v| v.len() >= 3);
        map
    }

    fn run_melds(&self) -> HashMap<Suit, Vec<&Card>> {
        let mut suit_map: HashMap<Suit, Vec<&Card>> = HashMap::new();
        let mut map: HashMap<Suit, Vec<&Card>> = HashMap::new();

        for c in &self.cards {
            suit_map.entry(c.suit).or_insert_with(|| Vec::new()).push(c);
        }

        // only retain sets with 3 or more items in them
        // this isn't strictly necessary in this case, but may speed things up?
        // map.retain(|_, v| v.len() >= 3);

        for (suit, cards) in suit_map {
            let mut sorted = cards.clone();
            sorted.sort_by(|a, b| a.rank.cmp(&b.rank));

            let mut current_run: Vec<&Card> = Vec::new();
            for i in 0..sorted.len() {
                if current_run.is_empty() {
                    current_run.push(sorted[i]);
                } else {
                    // check last card on current_run vector
                    // if it is one rank below current [i], add to current_run
                    if current_run.last().unwrap().rank as i32 + 1 == sorted[i].rank as i32 {
                        current_run.push(sorted[i]);
                    // otherwise, check length of current_run
                    } else {
                        if current_run.len() >= 3 {
                            // put this somewhere
                            map.entry(suit)
                                .or_insert_with(|| Vec::new())
                                .extend(current_run.clone());
                        } else {
                            // clear current_run
                            current_run.clear();
                            current_run.push(sorted[i]);
                        }
                    }
                }
            }
        }

        map
    }

    fn get_melds(&self) -> (HashMap<Suit, Vec<&Card>>, HashMap<Rank, Vec<&Card>>) {
        // we need a function that accounts for a card potentially being in >1 meld
        // this function will return the run_melds and the set_melds for consideration in the
        // card_in_meld function
        let mut run_melds = self.run_melds();
        let mut set_melds = self.set_melds();

        for card in &self.cards {
            if let Some(r) = run_melds.get(&card.suit) {
                if let Some(s) = set_melds.get(&card.rank) {
                    // &card is part of both a set meld and a run meld
                }
            }
        }
    }

    fn card_in_meld(&self, card: &Card) -> bool {
        let melds = self.run_melds();

        // if let Some(m) = melds.get(&card.rank) {
        if let Some(m) = melds.get(&card.suit) {
            m.contains(&card)
        } else {
            false
        }
    }

    pub fn score(&self) -> i32 {
        let mut score = 0;

        for card in &self.cards {
            if self.card_in_meld(&card) {
                score += 0;
            } else {
                score += card.score();
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
