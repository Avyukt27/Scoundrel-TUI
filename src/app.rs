use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Styled, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget},
};

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
                .set_style(self.theme_colours.title)
                .bold()
                .centered()
                .block(
                    Block::bordered()
                        .borders(Borders::ALL)
                        .border_type(ratatui::widgets::BorderType::Thick)
                        .border_style(self.theme_colours.title),
                ),
            chunks.header,
        );
        frame.render_widget(
            Block::bordered()
                .title(
                    Line::from(" Deck ")
                        .italic()
                        .left_aligned()
                        .set_style(self.theme_colours.deck),
                )
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(self.theme_colours.deck),
            chunks.deck,
        );
        frame.render_widget(
            Block::bordered()
                .title(
                    Line::from(" Room ")
                        .italic()
                        .left_aligned()
                        .set_style(self.theme_colours.room),
                )
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(self.theme_colours.room),
            chunks.room,
        );
        frame.render_widget(
            Block::bordered()
                .title(
                    Line::from(" Current Weapon ")
                        .italic()
                        .left_aligned()
                        .set_style(self.theme_colours.weapon),
                )
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(self.theme_colours.weapon),
            chunks.current_weapon,
        );
        frame.render_widget(
            Block::bordered()
                .title(
                    Line::from(" Last Enemy Defeated with a Weapon ")
                        .italic()
                        .left_aligned()
                        .set_style(self.theme_colours.last_enemy),
                )
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(self.theme_colours.last_enemy),
            chunks.last_defeated_enemy,
        );
        frame.render_widget(
            Block::bordered()
                .title(
                    Line::from(" Discard ")
                        .italic()
                        .left_aligned()
                        .set_style(self.theme_colours.discard),
                )
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(self.theme_colours.discard),
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
}

struct UiLayout {
    header: Rect,
    deck: Rect,
    room: Rect,
    current_weapon: Rect,
    last_defeated_enemy: Rect,
    discard: Rect,
}

#[derive(Debug)]
struct ThemeColours {
    title: Color,
    deck: Color,
    room: Color,
    weapon: Color,
    last_enemy: Color,
    discard: Color,
}

impl ThemeColours {
    pub fn dungeon() -> Self {
        Self {
            title: Color::Rgb(180, 40, 45),
            deck: Color::Rgb(92, 64, 51),
            room: Color::Rgb(70, 68, 60),
            weapon: Color::Rgb(200, 200, 200),
            last_enemy: Color::Rgb(160, 60, 60),
            discard: Color::Rgb(90, 90, 90),
        }
    }
}
