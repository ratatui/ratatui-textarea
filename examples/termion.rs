use ratatui_core::terminal::Terminal;
use ratatui_termion::TermionBackend;
use ratatui_termion::termion::event::Event as TermEvent;
use ratatui_termion::termion::input::{MouseTerminal, TermRead};
use ratatui_termion::termion::raw::IntoRawMode;
use ratatui_termion::termion::screen::IntoAlternateScreen;
use ratatui_textarea::{Input, Key, TextArea};
use ratatui_widgets::block::Block;
use ratatui_widgets::borders::Borders;
use std::error::Error;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

enum Event {
    Term(TermEvent),
    Tick,
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?.into_alternate_screen()?;
    let stdout = MouseTerminal::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let events = {
        let events = io::stdin().events();
        let (tx, rx) = mpsc::channel();
        let keys_tx = tx.clone();
        thread::spawn(move || {
            for event in events.flatten() {
                keys_tx.send(Event::Term(event)).unwrap();
            }
        });
        thread::spawn(move || {
            loop {
                tx.send(Event::Tick).unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        });
        rx
    };

    let mut textarea = TextArea::default();
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("Termion Minimal Example"),
    );

    loop {
        match events.recv()? {
            Event::Term(event) => match event.into() {
                Input { key: Key::Esc, .. } => break,
                input => {
                    textarea.input(input);
                }
            },
            Event::Tick => {}
        }
        term.draw(|f| {
            f.render_widget(&textarea, f.area());
        })?;
    }

    drop(term); // Leave terminal raw mode to print the following line
    println!("Lines: {:?}", textarea.lines());
    Ok(())
}
