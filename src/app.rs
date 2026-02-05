use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use crate::scenes::GameScreen;
use crate::{card::Card, scenes::MainMenu};
use crate::{card::Deck, colours::ThemeColours};

#[derive(Debug)]
pub struct App {
    mode: Mode,
    exit: bool,
    theme_colours: ThemeColours,
    deck: Deck,
    weapon: Option<Card>,
    discard: Option<Card>,
    last_enemy: Option<Card>,
    room: Option<Vec<Card>>,
}

impl App {
    pub fn new() -> Self {
        let mut deck = Deck::dungeon();
        deck.shuffle();

        Self {
            mode: Mode::Game,
            exit: false,
            theme_colours: ThemeColours::dungeon(),
            deck,
            weapon: None,
            discard: None,
            last_enemy: None,
            room: None,
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
        match self.mode {
            Mode::MainMenu => {
                let main_menu = MainMenu {
                    theme_colours: self.theme_colours.clone(),
                };
                frame.render_widget(&main_menu, frame.area());
            }
            Mode::Game => {
                let game_screen = GameScreen {
                    theme_colours: self.theme_colours.clone(),
                    weapon: self.weapon.clone(),
                    discard: self.discard.clone(),
                    last_enemy: self.last_enemy.clone(),
                    room: self.room.clone(),
                };
                frame.render_widget(&game_screen, frame.area());
            }
        }
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
}

#[derive(Debug)]
enum Mode {
    MainMenu,
    Game,
}
