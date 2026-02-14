use ratatui_core::terminal::Terminal;
use ratatui_termwiz::TermwizBackend;
use ratatui_termwiz::termwiz::input::InputEvent;
use ratatui_termwiz::termwiz::terminal::Terminal as _;
use ratatui_textarea::{Input, Key, TextArea};
use ratatui_widgets::block::Block;
use ratatui_widgets::borders::Borders;
use std::error::Error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let backend = TermwizBackend::new()?;
    let mut term = Terminal::new(backend)?;
    term.hide_cursor()?;

    let mut textarea = TextArea::default();
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("Termwiz Minimal Example"),
    );

    // The event loop
    loop {
        term.draw(|f| {
            f.render_widget(&textarea, f.area());
        })?;

        if let Some(input) = term
            .backend_mut()
            .buffered_terminal_mut()
            .terminal()
            .poll_input(Some(Duration::from_millis(100)))?
        {
            if let InputEvent::Resized { cols, rows } = input {
                term.backend_mut()
                    .buffered_terminal_mut()
                    .resize(cols, rows);
            } else {
                match input.into() {
                    Input { key: Key::Esc, .. } => break,
                    input => {
                        textarea.input(input);
                    }
                }
            }
        }
    }

    term.show_cursor()?;
    term.flush()?;
    drop(term); // Leave terminal raw mode to print the following line

    println!("Lines: {:?}", textarea.lines());
    Ok(())
}
