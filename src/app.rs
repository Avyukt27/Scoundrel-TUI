use std::{io, rc::Rc};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget},
};

#[derive(Default, Debug)]
pub struct App {
    counter: i8,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let chunks = self.get_layout(frame);

        let title = Line::from(" Example Title ".yellow().bold());
        let body_left_block_title = Line::from(" Left block title ".red().italic());
        let body_right_block_title = Line::from(" Right block title ".green().italic());

        frame.render_widget(
            Block::bordered()
                .title(title.centered())
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Double),
            chunks.header,
        );
        frame.render_widget(
            Block::bordered()
                .title(body_left_block_title.left_aligned())
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded),
            chunks.body_left,
        );
        frame.render_widget(
            Block::bordered()
                .title(body_right_block_title.left_aligned())
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded),
            chunks.body_right,
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
            .constraints([Constraint::Percentage(15), Constraint::Min(0)])
            .split(frame.area());
        let body = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(header[1]);

        UiLayout {
            header: header[0],
            body_left: body[0],
            body_right: body[1],
        }
    }
}

struct UiLayout {
    header: Rect,
    body_left: Rect,
    body_right: Rect,
}
