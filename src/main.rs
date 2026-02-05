use std::io;

mod app;
mod card;
mod colours;
mod scenes;
mod widgets;

fn main() -> io::Result<()> {
    let mut app = app::App::new();
    ratatui::run(|terminal| app.run(terminal))
}
