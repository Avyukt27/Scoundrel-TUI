use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Block, Widget},
};

use crate::{
    card::{Rank, Suit},
    colours::SuitColours,
};

pub struct CardWidget {
    pub rank: Rank,
    pub suit: Suit,
    pub suit_colours: SuitColours,
}

impl Widget for CardWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (symbol, colour) = match self.suit {
            Suit::Hearts => ("", self.suit_colours.hearts),
            Suit::Diamonds => ("󰣏", self.suit_colours.diamonds),
            Suit::Spades => ("󰣑", self.suit_colours.spades),
            Suit::Clubs => ("󰣎", self.suit_colours.clubs),
        };

        let text = format!("{} {}", self.rank.display(), symbol);

        let x = area.x + (area.width.saturating_sub(text.len() as u16)) / 2;
        let y = (area.y + area.height / 2).saturating_sub(1);

        let block = Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(colour));
        block.render(area, buf);
        buf.set_string(x, y, text, Style::default().fg(colour));
    }
}
