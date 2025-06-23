//! # ratcl
//!
//! `ratcl` allows you to create complex ratatui layouts with a simple API.

use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

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
}
