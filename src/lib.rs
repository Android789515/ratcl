//! # ratcl
//!
//! `ratcl` allows you to create complex ratatui layouts with a simple API.

use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, widgets::Widget};

/// Defines the `LayoutCell` alias.
pub trait LayoutCell: Fn(Rect, &mut Buffer) {}

/// Implements the `LayoutCell` trait for its respective type.
impl <Type: Fn(Rect, &mut Buffer)> LayoutCell for Type {}

/// Creates a `LayoutCell` from a widget.
///
/// # Example
/// ```
/// use ratatui::{buffer::Buffer, layout::Rect, widgets::{Block, Widget}};
/// use ratcl::make_cell;
/// 
/// struct SomeStruct;
///
/// impl Widget for SomeStruct {
///     fn render(self, area: Rect, buffer: &mut Buffer) {
///         let some_other_widget = Block::default();
/// 
///         make_cell(some_other_widget)(area, buffer);
///     }
/// }
/// ```
pub fn make_cell(content: impl Widget + Clone) -> impl LayoutCell {
    move |rect, buffer| {
        content.clone()
            .render(rect, buffer);
    }
}

/// Creates a pair of rows with a given offset for the first row.
///
/// # Example
/// ```
/// use ratatui::{buffer::Buffer, layout::Rect, widgets::{Block, Paragraph, Widget}};
/// use ratcl::{rows, make_cell};
/// 
/// struct SomeStruct;
///
/// impl Widget for SomeStruct {
///     fn render(self, area: Rect, buffer: &mut Buffer) {
///         let some_block = Block::default();
///         let some_paragraph = Paragraph::new("Test")
///             .block(some_block);
/// 
///         rows(
///             make_cell(some_paragraph.clone()),
///             make_cell(some_paragraph),
///             0.5,
///         )(area, buffer);
///     }
/// }
/// ```
pub fn rows(top_cell: impl LayoutCell, bottom_cell: impl LayoutCell, offset: f64) -> impl LayoutCell {
    move |rect, buffer| {
        let offset_percent = (offset * 100.0) as u16;

        let rects = Layout::vertical([
            Constraint::Percentage(offset_percent),
            Constraint::Percentage(100 - offset_percent),
        ]).split(rect);

        top_cell(rects[0], buffer);
        bottom_cell(rects[1], buffer);
    }
}

#[cfg(test)]
mod tests {
    use ratatui::widgets::Paragraph;

    use super::*;

    fn setup_test(word: &str, buffer_len: u16, buffer_height: u16) -> ( Buffer, Paragraph ) {
        let buffer = Buffer::empty(Rect::new(0, 0, buffer_len, buffer_height));

        let paragraph = Paragraph::new(word);

        ( buffer, paragraph )
    }

    #[test]
    fn renders_cell() {
        let word = "Hello";

        let ( mut buffer, widget ) = setup_test(word, word.len() as u16, 1);

        make_cell(widget)(buffer.area, &mut buffer);

        let expected_buffer = Buffer::with_lines(vec![
            word,
        ]);

        assert_eq!(buffer, expected_buffer);
    }

    #[test]
    fn creates_rows() {
        let word = "Hello";

        let ( mut buffer, widget ) = setup_test(word, 5, 2);

        rows(
            make_cell(widget.clone()),
            make_cell(widget),
            0.5,
        )(buffer.area, &mut buffer);

        let expected_buffer = Buffer::with_lines(vec![
            "Hello",
            "Hello",
        ]);

        assert_eq!(buffer, expected_buffer);
    }
}
