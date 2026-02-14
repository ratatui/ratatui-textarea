use ratatui_core::layout::Rect;
use ratatui_core::style::{Color, Style};
use ratatui_core::terminal::Terminal;
use ratatui_crossterm::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui_crossterm::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui_crossterm::{CrosstermBackend, crossterm};
use ratatui_textarea::{Input, Key, TextArea};
use ratatui_widgets::block::Block;
use ratatui_widgets::borders::Borders;
use std::io;

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut textarea = TextArea::default();
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::LightBlue))
            .title("Crossterm Popup Example"),
    );

    let area = Rect {
        width: 40,
        height: 5,
        x: 5,
        y: 5,
    };
    textarea.set_style(Style::default().fg(Color::Yellow));
    textarea.set_placeholder_style(Style::default());
    textarea.set_placeholder_text("prompt message");
    loop {
        term.draw(|f| {
            f.render_widget(&textarea, area);
        })?;
        match crossterm::event::read()?.into() {
            Input { key: Key::Esc, .. } => break,
            input => {
                textarea.input(input);
            }
        }
    }

    disable_raw_mode()?;
    crossterm::execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    term.show_cursor()?;

    println!("Lines: {:?}", textarea.lines());
    Ok(())
}
