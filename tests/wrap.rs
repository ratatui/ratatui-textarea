use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::style::{Color, Style};
use ratatui_core::widgets::Widget as _;
use ratatui_textarea::{CursorMove, TextArea, WrapMode};

fn render_lines(textarea: &TextArea<'_>, width: u16, height: u16) -> Vec<String> {
    let area = Rect {
        x: 0,
        y: 0,
        width,
        height,
    };
    let mut buf = Buffer::empty(area);
    textarea.render(area, &mut buf);

    (0..height)
        .map(|y| {
            let mut line = String::new();
            for x in 0..width {
                line.push_str(buf[(x, y)].symbol());
            }
            line
        })
        .collect()
}

fn render_buffer(textarea: &TextArea<'_>, width: u16, height: u16) -> Buffer {
    let area = Rect {
        x: 0,
        y: 0,
        width,
        height,
    };
    let mut buf = Buffer::empty(area);
    textarea.render(area, &mut buf);
    buf
}

#[test]
fn wrap_mode_default_is_none() {
    let textarea = TextArea::default();
    assert_eq!(textarea.wrap_mode(), WrapMode::None);
}

#[test]
fn none_mode_keeps_horizontal_behavior() {
    let textarea = TextArea::from(["abcdef"]);
    let lines = render_lines(&textarea, 5, 2);
    assert_eq!(lines, vec!["abcde".to_string(), "     ".to_string()]);
}

#[test]
fn word_or_glyph_mode_soft_wraps() {
    let mut textarea = TextArea::from(["abcdef"]);
    textarea.set_wrap_mode(WrapMode::WordOrGlyph);
    let lines = render_lines(&textarea, 5, 2);
    assert_eq!(lines, vec!["abcde".to_string(), "f    ".to_string()]);
}

#[test]
fn word_mode_does_not_split_long_word() {
    let mut textarea = TextArea::from(["abcdefgh"]);
    textarea.set_wrap_mode(WrapMode::Word);
    let lines = render_lines(&textarea, 5, 2);
    assert_eq!(lines, vec!["abcde".to_string(), "     ".to_string()]);
}

#[test]
fn word_mode_wraps_with_newlines() {
    let mut textarea = TextArea::from(["hello world", "again here"]);
    textarea.set_wrap_mode(WrapMode::Word);

    let lines = render_lines(&textarea, 6, 4);
    assert_eq!(
        lines,
        vec![
            "hello ".to_string(),
            "world ".to_string(),
            "again ".to_string(),
            "here  ".to_string(),
        ]
    );
}

#[test]
fn none_mode_long_multiline_text_stays_unwrapped() {
    let textarea = TextArea::from(["first very long line", "second very long line"]);
    let lines = render_lines(&textarea, 8, 4);
    assert_eq!(
        lines,
        vec![
            "first ve".to_string(),
            "second v".to_string(),
            "        ".to_string(),
            "        ".to_string(),
        ]
    );
}

#[test]
fn word_mode_long_paragraph_wraps_by_words() {
    let mut textarea = TextArea::from(["lorem ipsum dolor sit amet consectetur adipiscing elit"]);
    textarea.set_wrap_mode(WrapMode::Word);
    let lines = render_lines(&textarea, 12, 6);
    assert_eq!(
        lines,
        vec![
            "lorem ipsum ".to_string(),
            "dolor sit   ".to_string(),
            "amet        ".to_string(),
            "consectetur ".to_string(),
            "adipiscing  ".to_string(),
            "elit        ".to_string(),
        ]
    );
}

#[test]
fn glyph_mode_splits_long_token() {
    let mut textarea = TextArea::from(["0123456789abcdefghijklmnopqrstuvwxyz"]);
    textarea.set_wrap_mode(WrapMode::Glyph);
    let lines = render_lines(&textarea, 10, 4);
    assert_eq!(
        lines,
        vec![
            "0123456789".to_string(),
            "abcdefghij".to_string(),
            "klmnopqrst".to_string(),
            "uvwxyz    ".to_string(),
        ]
    );
}

#[test]
fn word_and_word_or_glyph_differ_for_long_words() {
    let text = "alpha supercalifragilisticexpialidocious omega";

    let mut word = TextArea::from([text]);
    word.set_wrap_mode(WrapMode::Word);
    let word_lines = render_lines(&word, 10, 4);
    assert_eq!(
        word_lines,
        vec![
            "alpha     ".to_string(),
            "supercalif".to_string(),
            " omega    ".to_string(),
            "          ".to_string(),
        ]
    );

    let mut word_or_glyph = TextArea::from([text]);
    word_or_glyph.set_wrap_mode(WrapMode::WordOrGlyph);
    let word_or_glyph_lines = render_lines(&word_or_glyph, 10, 6);
    assert_eq!(
        word_or_glyph_lines,
        vec![
            "alpha     ".to_string(),
            "supercalif".to_string(),
            "ragilistic".to_string(),
            "expialidoc".to_string(),
            "ious      ".to_string(),
            " omega    ".to_string(),
        ]
    );
}

#[test]
fn wrapped_mode_line_numbers_on_continuation_rows() {
    let mut textarea = TextArea::from(["abcdefghijk", "xy"]);
    textarea.set_wrap_mode(WrapMode::WordOrGlyph);
    textarea.set_line_number_style(Style::default());

    let lines = render_lines(&textarea, 8, 4);
    assert_eq!(
        lines,
        vec![
            " 1 abcde".to_string(),
            "   fghij".to_string(),
            "   k    ".to_string(),
            " 2 xy   ".to_string(),
        ]
    );
}

#[test]
fn wrapped_mode_handles_tiny_width() {
    let mut textarea = TextArea::from(["abcd"]);
    textarea.set_wrap_mode(WrapMode::WordOrGlyph);

    let lines = render_lines(&textarea, 1, 4);
    assert_eq!(
        lines,
        vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ]
    );
}

#[test]
fn wrapped_mode_preserves_empty_logical_lines() {
    let mut textarea = TextArea::from(["ab", "", "cd"]);
    textarea.set_wrap_mode(WrapMode::WordOrGlyph);

    let lines = render_lines(&textarea, 2, 3);
    assert_eq!(
        lines,
        vec!["ab".to_string(), "  ".to_string(), "cd".to_string()]
    );
}

#[test]
fn glyph_mode_combining_grapheme_renders_in_two_rows() {
    let mut textarea = TextArea::from(["e\u{301}x"]);
    textarea.set_wrap_mode(WrapMode::Glyph);

    let lines = render_lines(&textarea, 1, 2);
    assert_eq!(lines, vec!["e".to_string(), "x".to_string()]);
}

#[test]
fn wrapped_mode_scrolls_to_cursor_visual_row() {
    let mut textarea = TextArea::from(["abcdefghijklmnopqrstuvwxyz"]);
    textarea.set_wrap_mode(WrapMode::WordOrGlyph);
    textarea.move_cursor(CursorMove::End);

    let lines = render_lines(&textarea, 5, 2);
    assert_eq!(lines, vec!["uvwxy".to_string(), "z    ".to_string()]);
}

#[test]
fn wrapped_mode_with_tab_does_not_hide_following_text() {
    let mut textarea = TextArea::from(["\tX"]);
    textarea.set_wrap_mode(WrapMode::WordOrGlyph);
    let lines = render_lines(&textarea, 2, 2);
    assert_eq!(lines, vec!["  ".to_string(), "X ".to_string()]);
}

#[test]
fn cursor_line_style_applies_to_wrapped_continuation_rows() {
    let mut textarea = TextArea::from(["abcdefghij"]);
    textarea.set_wrap_mode(WrapMode::WordOrGlyph);
    let style = Style::default().bg(Color::Red);
    textarea.set_cursor_line_style(style);

    let buf = render_buffer(&textarea, 5, 2);
    assert_eq!(buf[(1, 1)].style().bg, style.bg);
}

#[test]
fn selection_does_not_extend_with_synthetic_space_on_wrap_boundary() {
    let mut textarea = TextArea::from(["hello world again"]);
    textarea.set_wrap_mode(WrapMode::Word);
    textarea.set_cursor_line_style(Style::default());
    let select_style = Style::default().bg(Color::Blue);
    textarea.set_selection_style(select_style);
    textarea.start_selection();
    textarea.move_cursor(CursorMove::End);

    let buf = render_buffer(&textarea, 10, 3);
    assert_eq!(buf[(5, 0)].style().bg, select_style.bg);
    assert_ne!(buf[(6, 0)].style().bg, select_style.bg);
}

fn render(textarea: &TextArea<'_>, width: u16, height: u16) {
    let area = Rect {
        x: 0,
        y: 0,
        width,
        height,
    };
    let mut buf = Buffer::empty(area);
    textarea.render(area, &mut buf);
}

#[test]
fn wrapped_cursor_down_moves_within_same_logical_line() {
    let mut textarea = TextArea::from(["abcdefghij"]);
    textarea.set_wrap_mode(WrapMode::WordOrGlyph);
    render(&textarea, 5, 4);

    assert_eq!(textarea.cursor(), (0, 0));
    textarea.move_cursor(CursorMove::Down);
    assert_eq!(textarea.cursor(), (0, 5));
    textarea.move_cursor(CursorMove::Down);
    assert_eq!(textarea.cursor(), (0, 5));
}

#[test]
fn wrapped_cursor_up_moves_within_same_logical_line() {
    let mut textarea = TextArea::from(["abcdefghij"]);
    textarea.set_wrap_mode(WrapMode::WordOrGlyph);
    render(&textarea, 5, 4);

    textarea.move_cursor(CursorMove::End);
    assert_eq!(textarea.cursor(), (0, 10));
    textarea.move_cursor(CursorMove::Up);
    assert_eq!(textarea.cursor(), (0, 4));
    textarea.move_cursor(CursorMove::Up);
    assert_eq!(textarea.cursor(), (0, 4));
}

#[test]
fn wrapped_cursor_down_crosses_logical_line_boundary() {
    let mut textarea = TextArea::from(["abcdefghij", "xy"]);
    textarea.set_wrap_mode(WrapMode::WordOrGlyph);
    render(&textarea, 5, 4);

    assert_eq!(textarea.cursor(), (0, 0));
    textarea.move_cursor(CursorMove::Down);
    assert_eq!(textarea.cursor(), (0, 5));
    textarea.move_cursor(CursorMove::Down);
    assert_eq!(textarea.cursor(), (1, 0));
    textarea.move_cursor(CursorMove::Down);
    assert_eq!(textarea.cursor(), (1, 0));
}

#[test]
fn wrapped_cursor_up_crosses_logical_line_boundary() {
    let mut textarea = TextArea::from(["abcdefghij", "xy"]);
    textarea.set_wrap_mode(WrapMode::WordOrGlyph);
    render(&textarea, 5, 4);

    textarea.move_cursor(CursorMove::Jump(1, 0));
    assert_eq!(textarea.cursor(), (1, 0));
    textarea.move_cursor(CursorMove::Up);
    assert_eq!(textarea.cursor(), (0, 5));
    textarea.move_cursor(CursorMove::Up);
    assert_eq!(textarea.cursor(), (0, 0));
    textarea.move_cursor(CursorMove::Up);
    assert_eq!(textarea.cursor(), (0, 0));
}

#[test]
fn wrapped_cursor_column_preserved_across_visual_lines() {
    let mut textarea = TextArea::from(["abcdefghij"]);
    textarea.set_wrap_mode(WrapMode::WordOrGlyph);
    render(&textarea, 5, 4);

    textarea.move_cursor(CursorMove::Jump(0, 2));
    assert_eq!(textarea.cursor(), (0, 2));
    textarea.move_cursor(CursorMove::Down);
    assert_eq!(textarea.cursor(), (0, 7));
}

#[test]
fn wrapped_cursor_column_clamped_to_shorter_visual_line() {
    let mut textarea = TextArea::from(["abcdefgh", "xy"]);
    textarea.set_wrap_mode(WrapMode::WordOrGlyph);
    render(&textarea, 5, 4);

    textarea.move_cursor(CursorMove::Jump(0, 4));
    assert_eq!(textarea.cursor(), (0, 4));
    textarea.move_cursor(CursorMove::Down);
    assert_eq!(textarea.cursor(), (0, 8));
}

#[test]
fn wrapped_cursor_down_preserves_visual_column_for_mixed_width_same_logical_line() {
    let mut textarea = TextArea::from(["a中bcde"]);
    textarea.set_wrap_mode(WrapMode::Glyph);
    render(&textarea, 4, 4);

    // After `a中`, the data column is 2 but the visual column is 3.
    textarea.move_cursor(CursorMove::Jump(0, 2));
    assert_eq!(textarea.cursor(), (0, 2));
    textarea.move_cursor(CursorMove::Down);
    assert_eq!(textarea.cursor(), (0, 6));
}

#[test]
fn wrapped_cursor_up_preserves_visual_column_for_mixed_width_same_logical_line() {
    let mut textarea = TextArea::from(["a中bcde"]);
    textarea.set_wrap_mode(WrapMode::Glyph);
    render(&textarea, 4, 4);

    textarea.move_cursor(CursorMove::Jump(0, 6));
    assert_eq!(textarea.cursor(), (0, 6));
    textarea.move_cursor(CursorMove::Up);
    assert_eq!(textarea.cursor(), (0, 2));
}

#[test]
fn wrapped_cursor_down_preserves_visual_column_when_crossing_into_mixed_width_line() {
    let mut textarea = TextArea::from(["abcd", "a中bcde"]);
    textarea.set_wrap_mode(WrapMode::Glyph);
    render(&textarea, 4, 4);

    // Visual column 3 in ASCII should land at visual column 3 in the mixed-width row.
    textarea.move_cursor(CursorMove::Jump(0, 3));
    assert_eq!(textarea.cursor(), (0, 3));
    textarea.move_cursor(CursorMove::Down);
    assert_eq!(textarea.cursor(), (1, 2));
}

#[test]
fn wrapped_cursor_up_preserves_visual_column_when_crossing_out_of_mixed_width_line() {
    let mut textarea = TextArea::from(["abcd", "a中bcde"]);
    textarea.set_wrap_mode(WrapMode::Glyph);
    render(&textarea, 4, 4);

    textarea.move_cursor(CursorMove::Jump(1, 2));
    assert_eq!(textarea.cursor(), (1, 2));
    textarea.move_cursor(CursorMove::Up);
    assert_eq!(textarea.cursor(), (0, 3));
}

#[test]
fn wrapped_cursor_down_up_with_word_wrap_mode() {
    let mut textarea = TextArea::from(["hello world"]);
    textarea.set_wrap_mode(WrapMode::Word);
    render(&textarea, 6, 4);

    assert_eq!(textarea.cursor(), (0, 0));
    textarea.move_cursor(CursorMove::Down);
    assert_eq!(textarea.cursor(), (0, 6));
    textarea.move_cursor(CursorMove::Up);
    assert_eq!(textarea.cursor(), (0, 0));
}

#[test]
fn wrapped_cursor_no_wrap_short_lines_behave_normally() {
    let mut textarea = TextArea::from(["abc", "def", "ghi"]);
    textarea.set_wrap_mode(WrapMode::WordOrGlyph);
    render(&textarea, 10, 4);

    assert_eq!(textarea.cursor(), (0, 0));
    textarea.move_cursor(CursorMove::Down);
    assert_eq!(textarea.cursor(), (1, 0));
    textarea.move_cursor(CursorMove::Down);
    assert_eq!(textarea.cursor(), (2, 0));
    textarea.move_cursor(CursorMove::Up);
    assert_eq!(textarea.cursor(), (1, 0));
    textarea.move_cursor(CursorMove::Up);
    assert_eq!(textarea.cursor(), (0, 0));
}

#[test]
fn wrapped_cursor_three_visual_lines_from_one_logical() {
    let mut textarea = TextArea::from(["abcdefghijklmno"]);
    textarea.set_wrap_mode(WrapMode::WordOrGlyph);
    render(&textarea, 5, 4);

    assert_eq!(textarea.cursor(), (0, 0));
    textarea.move_cursor(CursorMove::Down);
    assert_eq!(textarea.cursor(), (0, 5));
    textarea.move_cursor(CursorMove::Down);
    assert_eq!(textarea.cursor(), (0, 10));
    textarea.move_cursor(CursorMove::Down);
    assert_eq!(textarea.cursor(), (0, 10));
    textarea.move_cursor(CursorMove::Up);
    assert_eq!(textarea.cursor(), (0, 5));
    textarea.move_cursor(CursorMove::Up);
    assert_eq!(textarea.cursor(), (0, 0));
}
