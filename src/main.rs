use std::io;

use crate::card::{Card, Deck};

mod app;
mod card;

fn main() -> io::Result<()> {
    let mut deck = Deck::from_cards(vec![
        Card::new(card::Suit::Spades, card::Rank::Two),
        Card::new(card::Suit::Spades, card::Rank::Three),
        Card::new(card::Suit::Spades, card::Rank::Four),
        Card::new(card::Suit::Spades, card::Rank::Five),
        Card::new(card::Suit::Spades, card::Rank::Six),
        Card::new(card::Suit::Spades, card::Rank::Seven),
        Card::new(card::Suit::Spades, card::Rank::Eight),
        Card::new(card::Suit::Spades, card::Rank::Nine),
        Card::new(card::Suit::Spades, card::Rank::Ten),
        Card::new(card::Suit::Spades, card::Rank::Jack),
        Card::new(card::Suit::Spades, card::Rank::Queen),
        Card::new(card::Suit::Spades, card::Rank::King),
        Card::new(card::Suit::Spades, card::Rank::Ace),
        Card::new(card::Suit::Clubs, card::Rank::Two),
        Card::new(card::Suit::Clubs, card::Rank::Three),
        Card::new(card::Suit::Clubs, card::Rank::Four),
        Card::new(card::Suit::Clubs, card::Rank::Five),
        Card::new(card::Suit::Clubs, card::Rank::Six),
        Card::new(card::Suit::Clubs, card::Rank::Seven),
        Card::new(card::Suit::Clubs, card::Rank::Eight),
        Card::new(card::Suit::Clubs, card::Rank::Nine),
        Card::new(card::Suit::Clubs, card::Rank::Ten),
        Card::new(card::Suit::Clubs, card::Rank::Jack),
        Card::new(card::Suit::Clubs, card::Rank::Queen),
        Card::new(card::Suit::Clubs, card::Rank::King),
        Card::new(card::Suit::Clubs, card::Rank::Ace),
        Card::new(card::Suit::Diamonds, card::Rank::Two),
        Card::new(card::Suit::Diamonds, card::Rank::Three),
        Card::new(card::Suit::Diamonds, card::Rank::Four),
        Card::new(card::Suit::Diamonds, card::Rank::Five),
        Card::new(card::Suit::Diamonds, card::Rank::Six),
        Card::new(card::Suit::Diamonds, card::Rank::Seven),
        Card::new(card::Suit::Diamonds, card::Rank::Eight),
        Card::new(card::Suit::Diamonds, card::Rank::Nine),
        Card::new(card::Suit::Diamonds, card::Rank::Ten),
        Card::new(card::Suit::Hearts, card::Rank::Two),
        Card::new(card::Suit::Hearts, card::Rank::Three),
        Card::new(card::Suit::Hearts, card::Rank::Four),
        Card::new(card::Suit::Hearts, card::Rank::Five),
        Card::new(card::Suit::Hearts, card::Rank::Six),
        Card::new(card::Suit::Hearts, card::Rank::Seven),
        Card::new(card::Suit::Hearts, card::Rank::Eight),
        Card::new(card::Suit::Hearts, card::Rank::Nine),
        Card::new(card::Suit::Hearts, card::Rank::Ten),
    ]);

    let mut app = app::App::new();
    ratatui::run(|terminal| app.run(terminal))
}
