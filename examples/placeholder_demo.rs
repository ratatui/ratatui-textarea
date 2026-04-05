/// Demonstrates all placeholder API combinations:
///
///   Tab / Shift-Tab  — cycle between panes
///   r                — reset current pane (clear all text, restore original placeholder)
///   c                — clear placeholder on current pane
///   s                — set a plain-string placeholder on current pane
///   Ctrl-S           — set a rich styled-Text placeholder on current pane
///   Esc              — quit
use ratatui_core::layout::{Constraint, Direction, Layout};
use ratatui_core::style::{Color, Modifier, Style};
use ratatui_core::terminal::Terminal;
use ratatui_core::text::{Line, Span, Text};
use ratatui_crossterm::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui_crossterm::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui_crossterm::{CrosstermBackend, crossterm};
use ratatui_textarea::{Input, Key, TextArea};
use ratatui_widgets::block::Block;
use ratatui_widgets::borders::Borders;
use std::io;

// Each demo pane has a label and a factory that creates the textarea in its initial state.
struct Pane<'a> {
    label: &'static str,
    textarea: TextArea<'a>,
}

fn make_panes() -> Vec<Pane<'static>> {
    // --- 1. No placeholder ---
    let mut no_placeholder = TextArea::default();
    no_placeholder.set_block(pane_block("1. No placeholder"));

    // --- 2. Plain string placeholder (default DarkGray style) ---
    let mut plain = TextArea::default();
    plain.set_styled_placeholder("Enter your name…");
    plain.set_block(pane_block("2. Plain string (set_styled_placeholder)"));

    // --- 3. Single styled span ---
    let mut styled_span = TextArea::default();
    styled_span.set_styled_placeholder(Span::styled(
        "Required field",
        Style::default()
            .fg(Color::Red)
            .add_modifier(Modifier::ITALIC),
    ));
    styled_span.set_block(pane_block("3. Single styled Span"));

    // --- 4. Multi-span line (mixed styles in one line) ---
    let mut multi_span = TextArea::default();
    multi_span.set_styled_placeholder(Line::from(vec![
        Span::styled(
            "Email",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(": "),
        Span::styled("user@example.com", Style::default().fg(Color::DarkGray)),
    ]));
    multi_span.set_block(pane_block("4. Multi-span Line"));

    // --- 5. Multi-line Text placeholder ---
    let mut multiline = TextArea::default();
    multiline.set_styled_placeholder(Text::from_iter([
        Line::styled(
            "Line 1: Start typing here…",
            Style::default().fg(Color::Yellow),
        ),
        Line::styled(
            "Line 2: This is a multi-line placeholder",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::DIM),
        ),
        Line::styled(
            "Line 3: Each line styled independently",
            Style::default().fg(Color::DarkGray),
        ),
    ]));
    multiline.set_block(pane_block("5. Multi-line Text"));

    // --- 6. Placeholder with whole-Text style override ---
    let mut text_style = TextArea::default();
    text_style.set_styled_placeholder(
        Text::raw("Styled via Text.style (green bg)").style(Style::default().bg(Color::DarkGray)),
    );
    text_style.set_block(pane_block("6. Text.style override"));

    // --- 7. Plain API: set_placeholder_text + set_placeholder_style ---
    let mut legacy = TextArea::default();
    legacy.set_placeholder_text("set_placeholder_text + set_placeholder_style");
    legacy.set_placeholder_style(Style::default().fg(Color::Magenta));
    legacy.set_block(pane_block("7. set_placeholder_text/style"));

    // --- 8. Placeholder cleared with set_placeholder_text("") ---
    let mut cleared_legacy = TextArea::default();
    cleared_legacy.set_placeholder_text("Visible placeholder");
    cleared_legacy.set_placeholder_text(""); // disables placeholder (empty-string semantics)
    cleared_legacy.set_block(pane_block("8. Cleared via set_placeholder_text(\"\")"));

    vec![
        Pane {
            label: "no_placeholder",
            textarea: no_placeholder,
        },
        Pane {
            label: "plain",
            textarea: plain,
        },
        Pane {
            label: "styled_span",
            textarea: styled_span,
        },
        Pane {
            label: "multi_span",
            textarea: multi_span,
        },
        Pane {
            label: "multiline",
            textarea: multiline,
        },
        Pane {
            label: "text_style",
            textarea: text_style,
        },
        Pane {
            label: "legacy",
            textarea: legacy,
        },
        Pane {
            label: "cleared_legacy",
            textarea: cleared_legacy,
        },
    ]
}

fn pane_block(title: &str) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(title.to_owned())
}

fn active_block(title: &str) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::LightBlue))
        .title(format!("▶ {title}"))
}

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut panes = make_panes();
    let mut active: usize = 0;

    loop {
        let n = panes.len();
        term.draw(|f| {
            let area = f.area();

            // Help bar at the bottom
            let [main_area, help_area] = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .areas(area);

            let help = Line::from(vec![
                Span::styled("Tab", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": next  "),
                Span::styled("r", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": reset  "),
                Span::styled("c", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": clear placeholder  "),
                Span::styled("s", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": set plain  "),
                Span::styled("Ctrl-S", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": set styled  "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": quit"),
            ]);
            f.buffer_mut()
                .set_line(help_area.x, help_area.y, &help, help_area.width);

            // Lay out panes in a 4×2 grid
            let cols = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(50); 2])
                .split(main_area);

            let rows_per_col = n.div_ceil(2);
            let row_constraint =
                vec![Constraint::Percentage(100 / rows_per_col as u16); rows_per_col];

            let left_rows = Layout::default()
                .direction(Direction::Vertical)
                .constraints(row_constraint.clone())
                .split(cols[0]);
            let right_rows = Layout::default()
                .direction(Direction::Vertical)
                .constraints(row_constraint)
                .split(cols[1]);

            for (i, pane) in panes.iter_mut().enumerate() {
                let cell = if i < rows_per_col {
                    left_rows[i]
                } else {
                    right_rows[i - rows_per_col]
                };

                if i == active {
                    pane.textarea.set_block(active_block(pane.label));
                } else {
                    pane.textarea.set_block(pane_block(pane.label));
                }
                f.render_widget(&pane.textarea, cell);
            }
        })?;

        let raw = crossterm::event::read()?;
        let input: Input = raw.into();

        match input {
            Input { key: Key::Esc, .. } => break,

            // Shift-Tab: previous pane (must come before the general Tab arm)
            Input {
                key: Key::Tab,
                shift: true,
                ..
            } => {
                active = (active + n - 1) % n;
            }
            // Tab: next pane
            Input { key: Key::Tab, .. } => {
                active = (active + 1) % n;
            }

            // r: reset pane to its original state
            Input {
                key: Key::Char('r'),
                ..
            } => {
                let label = panes[active].label;
                let new_panes = make_panes();
                if let Some(fresh) = new_panes.into_iter().find(|p| p.label == label) {
                    panes[active] = fresh;
                }
            }

            // c: clear the placeholder
            Input {
                key: Key::Char('c'),
                ..
            } => {
                panes[active]
                    .textarea
                    .set_styled_placeholder(Text::default());
            }

            // Ctrl-S: set a rich styled placeholder (must come before the plain 's' arm)
            Input {
                key: Key::Char('s'),
                ctrl: true,
                ..
            } => {
                panes[active]
                    .textarea
                    .set_styled_placeholder(Line::from(vec![
                        Span::styled(
                            "Rich",
                            Style::default()
                                .fg(Color::Green)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::raw(" styled "),
                        Span::styled(
                            "placeholder",
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::ITALIC),
                        ),
                    ]));
            }

            // s: set a plain string placeholder
            Input {
                key: Key::Char('s'),
                ..
            } => {
                panes[active]
                    .textarea
                    .set_styled_placeholder("← plain string placeholder");
            }

            // Everything else: send to the active textarea
            input => {
                panes[active].textarea.input(input);
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

    println!("Final content of each pane:");
    for pane in &panes {
        println!("  {}: {:?}", pane.label, pane.textarea.lines());
    }
    Ok(())
}
