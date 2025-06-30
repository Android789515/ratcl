//! # ratcl
//!
//! `ratcl` allows you to create complex `ratatui` layouts with a simple API.

use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, widgets::Widget};

/// An empty widget.
///
/// Useful for laying out one widget per a constraint.
///
/// # Example
/// ```
/// use ratatui::{buffer::Buffer, layout::{Constraint, Rect}, widgets::{Block, Paragraph, Widget}};
/// use ratcl::{Columns, Rows, EmptyWidget};
/// 
/// struct SomeStruct;
///
/// impl Widget for SomeStruct {
///     fn render(self, area: Rect, buffer: &mut Buffer) {
///         let some_block = Block::default();
///         let some_paragraph = Paragraph::new("Test")
///             .block(some_block);
/// 
///         Columns(
///             some_paragraph.clone(),
///             Rows(
///                 some_paragraph,
///                 EmptyWidget,
///                 Constraint::Percentage(50),
///             ),
///             Constraint::Length(4),
///         ).render(area, buffer);
///     }
/// }
/// ```
#[derive(Clone)]
pub struct EmptyWidget;

impl Widget for EmptyWidget {
    fn render(self, _: Rect, _: &mut Buffer) {}
}

/// Creates a pair of rows with a given constraint for the first row.
///
/// # Example
/// ```
/// use ratatui::{buffer::Buffer, layout::{Constraint, Rect}, widgets::{Block, Paragraph, Widget}};
/// use ratcl::{Rows, Columns};
/// 
/// struct SomeStruct;
///
/// impl Widget for SomeStruct {
///     fn render(self, area: Rect, buffer: &mut Buffer) {
///         let some_block = Block::default();
///         let some_paragraph = Paragraph::new("Test")
///             .block(some_block);
/// 
///         Rows(
///             some_paragraph.clone(),
///             Columns(
///                 some_paragraph.clone(),
///                 some_paragraph,
///                 Constraint::Length(8),
///             ),
///             Constraint::Length(3),
///         ).render(area, buffer);
///     }
/// }
/// ```
pub struct Rows<TopContent: Widget, BottomContent: Widget>(
    pub TopContent,
    pub BottomContent,
    pub Constraint,
);

impl <TopContent: Widget, BottomContent: Widget> Widget for Rows<TopContent, BottomContent> {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let rects = Layout::vertical([
            self.2,
            Constraint::Fill(1),
        ]).split(area);

        self.0.render(rects[0], buffer);
        self.1.render(rects[1], buffer);
    }
}

/// Creates a pair of columns with a given scale factor for the first column.
///
/// # Example
/// ```
/// use ratatui::{buffer::Buffer, layout::{Constraint, Rect}, widgets::{Block, Paragraph, Widget}};
/// use ratcl::{Columns, Rows};
/// 
/// struct SomeStruct;
///
/// impl Widget for SomeStruct {
///     fn render(self, area: Rect, buffer: &mut Buffer) {
///         let some_block = Block::default();
///         let some_paragraph = Paragraph::new("Test")
///             .block(some_block);
/// 
///         Columns(
///             some_paragraph.clone(),
///             Rows(
///                 some_paragraph.clone(),
///                 Columns(
///                     some_paragraph.clone(),
///                     some_paragraph,
///                     Constraint::Ratio(1, 2),
///                 ),
///                 Constraint::Percentage(30),
///             ),
///             Constraint::Length(5),
///         ).render(area, buffer);
///     }
/// }
/// ```
pub struct Columns<LeftContent: Widget, RightContent: Widget>(
    pub LeftContent,
    pub RightContent,
    pub Constraint,
);

impl <LeftContent: Widget, RightContent: Widget> Widget for Columns<LeftContent, RightContent> {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let rects = Layout::horizontal([
            self.2,
            Constraint::Fill(1),
        ]).split(area);

        self.0.render(rects[0], buffer);
        self.1.render(rects[1], buffer);
    }
}

#[cfg(test)]
mod tests {
    use ratatui::{symbols::border, widgets::{Block, Paragraph}};

    use super::*;

    #[test]
    fn creates_rows() {
        let word = "Hello";

        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 6));
        let widget = Paragraph::new(word);

        Rows(
            widget.clone(),
            Columns(
                widget.clone(),
                widget,
                Constraint::Percentage(50),
            ),
            Constraint::Length(4),
        ).render(buffer.area, &mut buffer);

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
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 20));
        let widget = Block::bordered()
            .border_set(border::ROUNDED);

        Columns(
            widget.clone(),
            Rows(
                widget.clone(),
                Columns(
                    widget.clone(),
                    widget,
                    Constraint::Length(8),
                ),
                Constraint::Length(5),
            ),
            Constraint::Percentage(40),
        ).render(buffer.area, &mut buffer);

        let expected_buffer = Buffer::with_lines(vec![
            "╭──────╮╭──────────╮",
            "│      ││          │",
            "│      ││          │",
            "│      ││          │",
            "│      │╰──────────╯",
            "│      │╭──────╮╭──╮",
            "│      ││      ││  │",
            "│      ││      ││  │",
            "│      ││      ││  │",
            "│      ││      ││  │",
            "│      ││      ││  │",
            "│      ││      ││  │",
            "│      ││      ││  │",
            "│      ││      ││  │",
            "│      ││      ││  │",
            "│      ││      ││  │",
            "│      ││      ││  │",
            "│      ││      ││  │",
            "│      ││      ││  │",
            "╰──────╯╰──────╯╰──╯",
        ]);

        assert_eq!(buffer, expected_buffer);
        
    }
}
