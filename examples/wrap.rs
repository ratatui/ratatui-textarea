use ratatui_core::layout::{Constraint, Direction, Layout, Rect};
use ratatui_core::style::{Modifier, Style};
use ratatui_core::terminal::Terminal;
use ratatui_crossterm::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui_crossterm::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui_crossterm::{CrosstermBackend, crossterm};
use ratatui_textarea::{Input, Key, TextArea, WrapMode};
use ratatui_widgets::block::Block;
use ratatui_widgets::borders::Borders;
use std::io;

const DEMO_WIDTH: u16 = 80;
const DEMO_HEIGHT: u16 = 20;

fn textarea_with_mode(mode: WrapMode) -> TextArea<'static> {
    let mut textarea = TextArea::default();
    textarea.set_wrap_mode(mode);
    textarea
}

fn style_textarea(textarea: &mut TextArea<'_>, title: &str) {
    textarea.set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));
    textarea.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(format!(" {title} ")),
    );
}

fn centered_area(area: Rect, width: u16, height: u16) -> Rect {
    let width = width.min(area.width);
    let height = height.min(area.height);
    Rect::new(
        area.x + (area.width - width) / 2,
        area.y + (area.height - height) / 2,
        width,
        height,
    )
}

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut textareas = [
        textarea_with_mode(WrapMode::Glyph),
        textarea_with_mode(WrapMode::Word),
    ];
    style_textarea(&mut textareas[0], "Character Wrap");
    style_textarea(&mut textareas[1], "Word Wrap");

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref());

    loop {
        term.draw(|f| {
            let area = centered_area(f.area(), DEMO_WIDTH, DEMO_HEIGHT);
            let chunks = layout.split(area);
            for (textarea, chunk) in textareas.iter().zip(chunks.iter()) {
                f.render_widget(textarea, *chunk);
            }
        })?;

        match crossterm::event::read()?.into() {
            Input { key: Key::Esc, .. } => break,
            input => {
                textareas[0].input(input.clone());
                textareas[1].input(input);
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

    println!("Character wrap textarea: {:?}", textareas[0].lines());
    println!("Word wrap textarea: {:?}", textareas[1].lines());
    Ok(())
}
