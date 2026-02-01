use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Rect},
    style::{Styled, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

use crate::card::Card;
use crate::colours::ThemeColours;

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

        frame.render_widget(
            Paragraph::new(Line::from("SCOUNDREL"))
                .set_style(self.theme_colours.main.title)
                .bold()
                .centered()
                .block(
                    Block::bordered()
                        .borders(Borders::ALL)
                        .border_type(ratatui::widgets::BorderType::Thick)
                        .border_style(self.theme_colours.main.title),
                ),
            chunks.header,
        );
        frame.render_widget(
            Block::bordered()
                .title(
                    Line::from(" Deck ")
                        .italic()
                        .left_aligned()
                        .set_style(self.theme_colours.main.deck),
                )
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(self.theme_colours.main.deck),
            chunks.deck,
        );
        frame.render_widget(
            Block::bordered()
                .title(
                    Line::from(" Room ")
                        .italic()
                        .left_aligned()
                        .set_style(self.theme_colours.main.room),
                )
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(self.theme_colours.main.room),
            chunks.room,
        );
        frame.render_widget(
            self.get_card(Card::new(crate::card::Suit::Spades, crate::card::Rank::Ten))
                .block(
                    Block::bordered()
                        .title(
                            Line::from(" Current Weapon ")
                                .italic()
                                .left_aligned()
                                .set_style(self.theme_colours.main.weapon),
                        )
                        .borders(Borders::ALL)
                        .border_type(ratatui::widgets::BorderType::Rounded)
                        .border_style(self.theme_colours.main.weapon),
                ),
            chunks.current_weapon,
        );
        frame.render_widget(
            Block::bordered()
                .title(
                    Line::from(" Last Enemy Defeated with a Weapon ")
                        .italic()
                        .left_aligned()
                        .set_style(self.theme_colours.main.last_enemy),
                )
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(self.theme_colours.main.last_enemy),
            chunks.last_defeated_enemy,
        );
        frame.render_widget(
            Block::bordered()
                .title(
                    Line::from(" Discard ")
                        .italic()
                        .left_aligned()
                        .set_style(self.theme_colours.main.discard),
                )
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(self.theme_colours.main.discard),
            chunks.discard,
        );
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

    fn get_card(&self, card: Card) -> Paragraph<'static> {
        let rank_text = Line::from(format!("{:?}", card.rank as u8)).bold();
        Paragraph::new(rank_text).set_style(self.theme_colours.suit.colour_for(card.suit))
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
