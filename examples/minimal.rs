use ratatui_core::layout::{Position, Rect};
use ratatui_core::style::{Color, Style};
use ratatui_core::terminal::Terminal;
use ratatui_crossterm::crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, Event, MouseButton, MouseEvent, MouseEventKind,
};
use ratatui_crossterm::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui_crossterm::{CrosstermBackend, crossterm};
use ratatui_textarea::{CursorMove, DataCursor, Input, Key, TextArea};
use ratatui_widgets::block::Block;
use ratatui_widgets::borders::Borders;
use std::io;

const SAMPLE_TEXT: [&str; 4] = [
    "Click to move the cursor, drag to select, scroll to move the view.",
    "宽字符占两列，点击它的任意一半都会落在同一个字符上。",
    "한국어와 日本語のテキストも同じように動作します。",
    "Tabs\tand\tmixed 幅 width also map back to the right character.",
];

fn clicked_cursor(textarea: &TextArea<'_>, area: Rect, event: &MouseEvent) -> Option<CursorMove> {
    let inner = textarea.block().map_or(area, |block| block.inner(area));
    if !inner.contains(Position::new(event.column, event.row)) {
        return None;
    }

    let (top_row, top_col) = textarea.scroll_offset();
    let row = event.row - inner.y + top_row;
    let col = (event.column - inner.x).saturating_sub(textarea.line_number_width()) + top_col;

    let DataCursor(row, col) = textarea.screen_to_data(usize::from(row), usize::from(col));
    Some(CursorMove::Jump(row as u16, col as u16))
}

fn handle_mouse(textarea: &mut TextArea<'_>, area: Rect, event: MouseEvent) {
    match event.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            if let Some(cursor) = clicked_cursor(textarea, area, &event) {
                textarea.cancel_selection();
                textarea.move_cursor(cursor);
                textarea.start_selection();
            }
        }
        MouseEventKind::Drag(MouseButton::Left) => {
            if let Some(cursor) = clicked_cursor(textarea, area, &event) {
                textarea.move_cursor(cursor);
            }
        }
        MouseEventKind::Up(MouseButton::Left) => {
            if textarea
                .selection_range()
                .is_some_and(|(from, to)| from == to)
            {
                textarea.cancel_selection();
            }
        }
        _ => {
            textarea.input(event);
        }
    }
}

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut textarea = TextArea::from(SAMPLE_TEXT);
    textarea.set_line_number_style(Style::default().fg(Color::DarkGray));
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("Crossterm Minimal Example"),
    );

    let mut area = Rect::default();
    loop {
        term.draw(|f| {
            area = f.area();
            f.render_widget(&textarea, area);
        })?;
        match crossterm::event::read()? {
            Event::Mouse(event) => handle_mouse(&mut textarea, area, event),
            event => match event.into() {
                Input { key: Key::Esc, .. } => break,
                input => {
                    textarea.input(input);
                }
            },
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
