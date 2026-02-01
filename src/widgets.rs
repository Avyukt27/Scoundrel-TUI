use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Block, Widget},
};

use crate::card::{Rank, Suit};

pub struct CardWidget {
    pub rank: Rank,
    pub suit: Suit,
}

impl Widget for CardWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().border_type(ratatui::widgets::BorderType::Rounded);
        block.render(area, buf);

        let symbol = match self.suit {
            Suit::Hearts => "",
            Suit::Diamonds => "󰣏",
            Suit::Spades => "󰣑",
            Suit::Clubs => "󰣎",
        };

        let text = format!("{} {}", self.rank.display(), symbol);

        let x = area.x + (area.width.saturating_sub(text.len() as u16)) / 2;
        let y = (area.y + area.height / 2).saturating_sub(1);

        buf.set_string(
            x,
            y,
            text,
            Style::default().fg(ratatui::style::Color::White),
        );
    }
}
