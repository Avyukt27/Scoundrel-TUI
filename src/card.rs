use rand::{rng, seq::SliceRandom};
use std::cmp::Ordering;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rank {
    Two = 2,
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
    Ace,
}

impl Rank {
    pub fn display(self) -> &'static str {
        match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn from_cards(cards: Vec<Card>) -> Self {
        Deck { cards }
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn add_to_end(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn dungeon() -> Self {
        Deck::from_cards(vec![
            Card::new(crate::card::Suit::Spades, crate::card::Rank::Two),
            Card::new(crate::card::Suit::Spades, crate::card::Rank::Three),
            Card::new(crate::card::Suit::Spades, crate::card::Rank::Four),
            Card::new(crate::card::Suit::Spades, crate::card::Rank::Five),
            Card::new(crate::card::Suit::Spades, crate::card::Rank::Six),
            Card::new(crate::card::Suit::Spades, crate::card::Rank::Seven),
            Card::new(crate::card::Suit::Spades, crate::card::Rank::Eight),
            Card::new(crate::card::Suit::Spades, crate::card::Rank::Nine),
            Card::new(crate::card::Suit::Spades, crate::card::Rank::Ten),
            Card::new(crate::card::Suit::Spades, crate::card::Rank::Jack),
            Card::new(crate::card::Suit::Spades, crate::card::Rank::Queen),
            Card::new(crate::card::Suit::Spades, crate::card::Rank::King),
            Card::new(crate::card::Suit::Spades, crate::card::Rank::Ace),
            Card::new(crate::card::Suit::Clubs, crate::card::Rank::Two),
            Card::new(crate::card::Suit::Clubs, crate::card::Rank::Three),
            Card::new(crate::card::Suit::Clubs, crate::card::Rank::Four),
            Card::new(crate::card::Suit::Clubs, crate::card::Rank::Five),
            Card::new(crate::card::Suit::Clubs, crate::card::Rank::Six),
            Card::new(crate::card::Suit::Clubs, crate::card::Rank::Seven),
            Card::new(crate::card::Suit::Clubs, crate::card::Rank::Eight),
            Card::new(crate::card::Suit::Clubs, crate::card::Rank::Nine),
            Card::new(crate::card::Suit::Clubs, crate::card::Rank::Ten),
            Card::new(crate::card::Suit::Clubs, crate::card::Rank::Jack),
            Card::new(crate::card::Suit::Clubs, crate::card::Rank::Queen),
            Card::new(crate::card::Suit::Clubs, crate::card::Rank::King),
            Card::new(crate::card::Suit::Clubs, crate::card::Rank::Ace),
            Card::new(crate::card::Suit::Diamonds, crate::card::Rank::Two),
            Card::new(crate::card::Suit::Diamonds, crate::card::Rank::Three),
            Card::new(crate::card::Suit::Diamonds, crate::card::Rank::Four),
            Card::new(crate::card::Suit::Diamonds, crate::card::Rank::Five),
            Card::new(crate::card::Suit::Diamonds, crate::card::Rank::Six),
            Card::new(crate::card::Suit::Diamonds, crate::card::Rank::Seven),
            Card::new(crate::card::Suit::Diamonds, crate::card::Rank::Eight),
            Card::new(crate::card::Suit::Diamonds, crate::card::Rank::Nine),
            Card::new(crate::card::Suit::Diamonds, crate::card::Rank::Ten),
            Card::new(crate::card::Suit::Hearts, crate::card::Rank::Two),
            Card::new(crate::card::Suit::Hearts, crate::card::Rank::Three),
            Card::new(crate::card::Suit::Hearts, crate::card::Rank::Four),
            Card::new(crate::card::Suit::Hearts, crate::card::Rank::Five),
            Card::new(crate::card::Suit::Hearts, crate::card::Rank::Six),
            Card::new(crate::card::Suit::Hearts, crate::card::Rank::Seven),
            Card::new(crate::card::Suit::Hearts, crate::card::Rank::Eight),
            Card::new(crate::card::Suit::Hearts, crate::card::Rank::Nine),
            Card::new(crate::card::Suit::Hearts, crate::card::Rank::Ten),
        ])
    }
}
