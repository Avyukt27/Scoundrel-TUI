use ratatui::style::Color;

use crate::card::Suit;

#[derive(Debug)]
pub struct ThemeColours {
    pub main: MainColours,
    pub suit: SuitColours,
}

impl ThemeColours {
    pub fn dungeon() -> Self {
        Self {
            main: MainColours::dungeon(),
            suit: SuitColours::dungeon(),
        }
    }
}

#[derive(Debug)]
pub struct MainColours {
    pub title: Color,
    pub deck: Color,
    pub room: Color,
    pub weapon: Color,
    pub last_enemy: Color,
    pub discard: Color,
}

impl MainColours {
    pub fn dungeon() -> Self {
        Self {
            title: Color::Rgb(180, 40, 45),
            deck: Color::Rgb(92, 64, 51),
            room: Color::Rgb(70, 68, 60),
            weapon: Color::Rgb(185, 165, 165),
            last_enemy: Color::Rgb(160, 60, 60),
            discard: Color::Rgb(95, 95, 90),
        }
    }
}

#[derive(Debug)]
pub struct SuitColours {
    pub clubs: Color,
    pub diamonds: Color,
    pub hearts: Color,
    pub spades: Color,
}

impl SuitColours {
    pub fn dungeon() -> Self {
        Self {
            clubs: Color::Rgb(70, 120, 80),
            diamonds: Color::Rgb(180, 140, 70),
            hearts: Color::Rgb(170, 60, 60),
            spades: Color::Rgb(90, 110, 140),
        }
    }

    pub fn colour_for(&self, suit: Suit) -> Color {
        match suit {
            Suit::Clubs => self.clubs,
            Suit::Diamonds => self.diamonds,
            Suit::Hearts => self.hearts,
            Suit::Spades => self.spades,
        }
    }
}
