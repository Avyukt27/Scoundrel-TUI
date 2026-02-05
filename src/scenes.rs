use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::Stylize,
    style::{Style, Styled},
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

use crate::{card::Card, colours::ThemeColours, widgets::CardWidget};

#[derive(Debug)]
pub struct MainMenu {
    pub theme_colours: ThemeColours,
}

impl Widget for &MainMenu {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let title_block = Block::bordered()
            .border_type(ratatui::widgets::BorderType::HeavyTripleDashed)
            .border_style(Style::default().fg(self.theme_colours.main.title));

        let title = Paragraph::new(Line::from("SCOUNDREL"))
            .set_style(self.theme_colours.main.title)
            .bold()
            .centered()
            .block(title_block);

        title.render(area, buf);
    }
}

#[derive(Debug)]
pub struct GameScreen {
    pub theme_colours: ThemeColours,
    pub weapon: Option<Card>,
    pub discard: Option<Card>,
    pub last_enemy: Option<Card>,
    pub room: Option<Vec<Card>>,
}

impl GameScreen {
    fn get_game_layout(&self, area: Rect) -> UiLayout {
        let header = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Max(3), Constraint::Min(0)])
            .split(area);
        let row_1 = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(header[1]);
        let row_2 = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(row_1[0]);
        let row_3 = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                Constraint::Min(20),
                Constraint::Min(20),
                Constraint::Max(30),
            ])
            .split(row_1[1]);

        UiLayout {
            header: header[0],
            deck: row_2[0],
            room: row_2[1],
            current_weapon: row_3[0],
            last_defeated_enemy: row_3[1],
            discard: row_3[2],
        }
    }
}

impl Widget for &GameScreen {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let chunks = self.get_game_layout(area);

        let title_block = Block::bordered()
            .border_type(ratatui::widgets::BorderType::HeavyTripleDashed)
            .border_style(Style::default().fg(self.theme_colours.main.title));
        let deck_block = Block::bordered()
            .title(
                Line::from(" Deck ")
                    .italic()
                    .left_aligned()
                    .set_style(self.theme_colours.main.deck),
            )
            .border_type(ratatui::widgets::BorderType::Plain)
            .border_style(Style::default().fg(self.theme_colours.main.deck));
        let room_block = Block::bordered()
            .title(
                Line::from(" Room ")
                    .italic()
                    .left_aligned()
                    .set_style(self.theme_colours.main.room),
            )
            .border_type(ratatui::widgets::BorderType::Plain)
            .border_style(Style::default().fg(self.theme_colours.main.room));
        let weapon_block_outer = Block::bordered()
            .title(
                Line::from(" Current Weapon ")
                    .italic()
                    .left_aligned()
                    .set_style(self.theme_colours.main.weapon),
            )
            .border_type(ratatui::widgets::BorderType::Plain)
            .border_style(Style::default().fg(self.theme_colours.main.weapon));
        let last_enemy_block = Block::bordered()
            .title(
                Line::from(" Last Enemy Defeated with a Weapon ")
                    .italic()
                    .left_aligned()
                    .set_style(self.theme_colours.main.last_enemy),
            )
            .border_type(ratatui::widgets::BorderType::Plain)
            .border_style(Style::default().fg(self.theme_colours.main.last_enemy));
        let discard_block = Block::bordered()
            .title(
                Line::from(" Discard ")
                    .italic()
                    .left_aligned()
                    .set_style(self.theme_colours.main.discard),
            )
            .border_type(ratatui::widgets::BorderType::Plain)
            .border_style(Style::default().fg(self.theme_colours.main.discard));

        let weapon_inner = weapon_block_outer.inner(chunks.current_weapon);
        let weapon_inner_chunks = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(14),
                Constraint::Min(0),
            ])
            .split(weapon_inner);
        let weapon_inner_chunks = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(24),
                Constraint::Min(0),
            ])
            .split(weapon_inner_chunks[1]);

        let room_areas = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(24),
                Constraint::Length(24),
                Constraint::Length(24),
                Constraint::Length(24),
                Constraint::Min(0),
            ])
            .split(chunks.room);
        let room_areas = &room_areas[1..=4];
        let room_inner_areas: Vec<ratatui::layout::Rect> = room_areas
            .iter()
            .map(|area| {
                Layout::default()
                    .direction(ratatui::layout::Direction::Vertical)
                    .constraints([
                        Constraint::Min(0),
                        Constraint::Length(14),
                        Constraint::Min(0),
                    ])
                    .split(*area)[1]
            })
            .collect();

        let last_enemy_inner = last_enemy_block.inner(chunks.last_defeated_enemy);
        let last_enemy_chunks = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(14),
                Constraint::Min(0),
            ])
            .split(last_enemy_inner);
        let last_enemy_chunks = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(24),
                Constraint::Min(0),
            ])
            .split(last_enemy_chunks[1]);

        let discard_inner = discard_block.inner(chunks.discard);
        let discard_inner_chunks = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(14),
                Constraint::Min(0),
            ])
            .split(discard_inner);
        let discard_inner_chunks = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(24),
                Constraint::Min(0),
            ])
            .split(discard_inner_chunks[1]);

        let title = Paragraph::new(Line::from("SCOUNDREL"))
            .set_style(self.theme_colours.main.title)
            .bold()
            .centered()
            .block(title_block);

        title.render(chunks.header, buf);
        deck_block.render(chunks.deck, buf);

        room_block.render(chunks.room, buf);
        if let Some(room) = &self.room {
            for (card_in_room, area) in room.iter().zip(room_inner_areas.iter()) {
                let widget = CardWidget {
                    rank: card_in_room.rank,
                    suit: card_in_room.suit,
                    suit_colours: self.theme_colours.suit.clone(),
                };
                widget.render(*area, buf);
            }
        }

        weapon_block_outer.render(chunks.current_weapon, buf);
        if let Some(weapon) = &self.weapon {
            CardWidget {
                rank: weapon.rank,
                suit: weapon.suit,
                suit_colours: self.theme_colours.suit.clone(),
            }
            .render(weapon_inner_chunks[1], buf);
        }

        last_enemy_block.render(chunks.last_defeated_enemy, buf);
        if let Some(last_enemy) = &self.last_enemy {
            CardWidget {
                rank: last_enemy.rank,
                suit: last_enemy.suit,
                suit_colours: self.theme_colours.suit.clone(),
            }
            .render(last_enemy_chunks[1], buf);
        }

        discard_block.render(chunks.discard, buf);
        if let Some(discard) = &self.discard {
            CardWidget {
                rank: discard.rank,
                suit: discard.suit,
                suit_colours: self.theme_colours.suit.clone(),
            }
            .render(discard_inner_chunks[1], buf);
        }
    }
}

struct UiLayout {
    header: Rect,
    deck: Rect,
    room: Rect,
    current_weapon: Rect,
    last_defeated_enemy: Rect,
    discard: Rect,
}
