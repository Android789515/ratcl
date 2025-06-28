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

/// An empty cell.
///
/// Useful for laying out one cell per a constraint.
///
/// # Example
/// ```
/// use ratatui::{buffer::Buffer, layout::{Constraint, Rect}, widgets::{Block, Paragraph, Widget}};
/// use ratcl::{make_cell, columns, rows, EmptyCell};
/// 
/// struct SomeStruct;
///
/// impl Widget for SomeStruct {
///     fn render(self, area: Rect, buffer: &mut Buffer) {
///         let some_block = Block::default();
///         let some_paragraph = Paragraph::new("Test")
///             .block(some_block);
/// 
///         columns(
///             make_cell(some_paragraph.clone()),
///             rows(
///                 make_cell(some_paragraph.clone()),
///                 make_cell(EmptyCell),
///                 Constraint::Percentage(50),
///             ),
///             Constraint::Length(4),
///         )(area, buffer);
///     }
/// }
/// ```
#[derive(Clone)]
pub struct EmptyCell;

impl Widget for EmptyCell {
    fn render(self, _: Rect, _: &mut Buffer) {}
}

/// Creates a pair of rows with a given constraint for the first row.
///
/// # Example
/// ```
/// use ratatui::{buffer::Buffer, layout::{Constraint, Rect}, widgets::{Block, Paragraph, Widget}};
/// use ratcl::{rows, columns, make_cell};
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
///             columns(
///                 make_cell(some_paragraph.clone()),
///                 make_cell(some_paragraph),
///                 Constraint::Length(8),
///             ),
///             Constraint::Length(3),
///         )(area, buffer);
///     }
/// }
/// ```
pub fn rows(top_cell: impl LayoutCell, bottom_cell: impl LayoutCell, constraint: Constraint) -> impl LayoutCell {
    move |rect, buffer| {
        let rects = Layout::vertical([
            constraint,
            Constraint::Fill(1),
        ]).split(rect);

        top_cell(rects[0], buffer);
        bottom_cell(rects[1], buffer);
    }
}

/// Creates a pair of columns with a given scale factor for the first column.
///
/// # Example
/// ```
/// use ratatui::{buffer::Buffer, layout::{Constraint, Rect}, widgets::{Block, Paragraph, Widget}};
/// use ratcl::{columns, rows, make_cell};
/// 
/// struct SomeStruct;
///
/// impl Widget for SomeStruct {
///     fn render(self, area: Rect, buffer: &mut Buffer) {
///         let some_block = Block::default();
///         let some_paragraph = Paragraph::new("Test")
///             .block(some_block);
/// 
///         columns(
///             make_cell(some_paragraph.clone()),
///             rows(
///                 make_cell(some_paragraph.clone()),
///                 columns(
///                     make_cell(some_paragraph.clone()),
///                     make_cell(some_paragraph),
///                     Constraint::Ratio(1, 2),
///                 ),
///                 Constraint::Percentage(30),
///             ),
///             Constraint::Length(5),
///         )(area, buffer);
///     }
/// }
/// ```
pub fn columns(left_cell: impl LayoutCell, right_cell: impl LayoutCell, constraint: Constraint) -> impl LayoutCell {
    move |rect, buffer| {
        let rects = Layout::horizontal([
            constraint,
            Constraint::Fill(1),
        ]).split(rect);

        left_cell(rects[0], buffer);
        right_cell(rects[1], buffer);
    }
}

#[cfg(test)]
mod tests {
    use ratatui::widgets::Paragraph;

    use super::*;

    fn setup_test_buffer(word: &str, buffer_len: u16, buffer_height: u16) -> ( Buffer, Paragraph ) {
        let buffer = Buffer::empty(Rect::new(0, 0, buffer_len, buffer_height));

        let paragraph = Paragraph::new(word);

        ( buffer, paragraph )
    }

    #[test]
    fn renders_cell() {
        let word = "Hello";

        let ( mut buffer, widget ) = setup_test_buffer(word, word.len() as u16, 1);

        make_cell(widget)(buffer.area, &mut buffer);

        let expected_buffer = Buffer::with_lines(vec![
            word,
        ]);

        assert_eq!(buffer, expected_buffer);
    }

    #[test]
    fn creates_rows() {
        let word = "Hello";

        let ( mut buffer, widget ) = setup_test_buffer(word, 10, 6);

        rows(
            make_cell(widget.clone()),
            columns(
                make_cell(widget.clone()),
                make_cell(widget.clone()),
                Constraint::Percentage(50),
            ),
            Constraint::Length(4),
        )(buffer.area, &mut buffer);

        let expected_buffer = Buffer::with_lines(vec![
            "Hello     ",
            "          ",
            "          ",
            "          ",
            "HelloHello",
            "          ",
        ]);

        assert_eq!(buffer, expected_buffer);
    }

    #[test]
    fn creates_columns() {
        let word = "Hello";

        let ( mut buffer, widget ) = setup_test_buffer(word, 10, 8);

        columns(
            make_cell(widget.clone()),
            rows(
                make_cell(widget.clone()),
                columns(
                    make_cell(widget.clone()),
                    make_cell(widget.clone()),
                    Constraint::Length(8),
                ),
                Constraint::Length(5),
            ),
            Constraint::Percentage(40),
        )(buffer.area, &mut buffer);

        let expected_buffer = Buffer::with_lines(vec![
            "HellHello ",
            "          ",
            "          ",
            "          ",
            "          ",
            "    Hello ",
            "          ",
            "          ",
        ]);

        assert_eq!(buffer, expected_buffer);
        
    }
}
