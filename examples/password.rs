use ratatui_core::layout::{Constraint, Layout};
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
    textarea.set_cursor_line_style(Style::default());
    textarea.set_mask_char('\u{2022}'); //U+2022 BULLET (•)
    textarea.set_placeholder_text("Please enter your password");
    let constraints = [Constraint::Length(3), Constraint::Min(1)];
    let layout = Layout::default().constraints(constraints);
    textarea.set_style(Style::default().fg(Color::LightGreen));
    textarea.set_block(Block::default().borders(Borders::ALL).title("Password"));

    loop {
        term.draw(|f| {
            let chunks = layout.split(f.area());
            f.render_widget(&textarea, chunks[0]);
        })?;

        match crossterm::event::read()?.into() {
            Input {
                key: Key::Esc | Key::Enter,
                ..
            } => break,
            input => {
                if textarea.input(input) {
                    // When the input modified its text, validate the text content
                }
            }
        }
    }

    disable_raw_mode()?;
    crossterm::execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    term.show_cursor()?;

    println!("Input: {:?}", textarea.lines()[0]);
    Ok(())
}
