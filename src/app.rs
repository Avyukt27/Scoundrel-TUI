use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Styled, Stylize},
    text::Line,
    widgets::{Block, Borders, Padding, Paragraph},
};

use crate::colours::ThemeColours;
use crate::{card::Card, widgets::CardWidget};

#[derive(Debug)]
pub struct App {
    exit: bool,
    theme_colours: ThemeColours,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            theme_colours: ThemeColours::dungeon(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let chunks = self.get_layout(frame);

        let title_block = Block::bordered()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Thick)
            .border_style(Style::default().fg(self.theme_colours.main.title));
        let deck_block = Block::bordered()
            .title(
                Line::from(" Deck ")
                    .italic()
                    .left_aligned()
                    .set_style(self.theme_colours.main.deck),
            )
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(self.theme_colours.main.deck));
        let room_block = Block::bordered()
            .title(
                Line::from(" Room ")
                    .italic()
                    .left_aligned()
                    .set_style(self.theme_colours.main.room),
            )
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(self.theme_colours.main.room));
        let weapon_block_outer = Block::bordered()
            .title(
                Line::from(" Current Weapon ")
                    .italic()
                    .left_aligned()
                    .set_style(self.theme_colours.main.weapon),
            )
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(self.theme_colours.main.weapon));
        let last_enemy_block = Block::bordered()
            .title(
                Line::from(" Last Enemy Defeated with a Weapon ")
                    .italic()
                    .left_aligned()
                    .set_style(self.theme_colours.main.last_enemy),
            )
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(self.theme_colours.main.last_enemy));
        let discard_block = Block::bordered()
            .title(
                Line::from(" Discard ")
                    .italic()
                    .left_aligned()
                    .set_style(self.theme_colours.main.discard),
            )
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(self.theme_colours.main.discard));
        let card_border = Block::bordered().border_type(ratatui::widgets::BorderType::Rounded);

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

        let title = Paragraph::new(Line::from("SCOUNDREL"))
            .set_style(self.theme_colours.main.title)
            .bold()
            .centered()
            .block(title_block);

        let cards = vec![
            Card::new(crate::card::Suit::Spades, crate::card::Rank::Eight),
            Card::new(crate::card::Suit::Clubs, crate::card::Rank::Five),
            Card::new(crate::card::Suit::Diamonds, crate::card::Rank::Ace),
            Card::new(crate::card::Suit::Hearts, crate::card::Rank::Ten),
        ];

        frame.render_widget(title, chunks.header);
        frame.render_widget(deck_block, chunks.deck);
        frame.render_widget(room_block, chunks.room);
        for (card_in_room, area) in cards.iter().zip(room_inner_areas.iter()) {
            let widget = CardWidget {
                rank: card_in_room.rank,
                suit: card_in_room.suit,
            };
            frame.render_widget(widget, *area);
        }
        frame.render_widget(weapon_block_outer, chunks.current_weapon);
        frame.render_widget(
            CardWidget {
                rank: crate::card::Rank::Nine,
                suit: crate::card::Suit::Clubs,
            },
            weapon_inner_chunks[1],
        );
        frame.render_widget(last_enemy_block, chunks.last_defeated_enemy);
        frame.render_widget(discard_block, chunks.discard);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            _ => {}
        }
    }

    fn get_layout(&self, frame: &mut Frame) -> UiLayout {
        let header = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Max(3), Constraint::Min(0)])
            .split(frame.area());
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

struct UiLayout {
    header: Rect,
    deck: Rect,
    room: Rect,
    current_weapon: Rect,
    last_defeated_enemy: Rect,
    discard: Rect,
}
