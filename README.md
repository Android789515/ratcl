# ratcl

Create complex layouts with a simple API.

## Create Rows

```rs
use ratatui::{buffer::Buffer, layout::Rect, widgets::{Paragraph, Widget}};

struct SomeStruct;

impl Widget for SomeStruct {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let paragraph = Paragraph::new("Hello");

        rows(
            make_cell(paragraph.clone()),
            make_cell(paragraph),
            0.5,
        )(area, buffer);
    }
}
```

## Create Columns

```rs
use ratatui::{buffer::Buffer, layout::Rect, widgets::{Paragraph, Widget}};

struct SomeStruct;

impl Widget for SomeStruct {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let paragraph = Paragraph::new("Hello");

        columns(
            make_cell(paragraph.clone()),
            make_cell(paragraph),
            0.5,
        )(area, buffer);
    }
}
```

## Create Complex Layouts

```rs
use ratatui::{buffer::Buffer, layout::Rect, widgets::{Paragraph, Widget}};

struct SomeStruct;

impl Widget for SomeStruct {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let paragraph = Paragraph::new("Hello");

        rows(
            make_cell(paragraph.clone()),
            rows(
              make_cell(paragraph.clone()),
              make_cell(paragraph),
              0.5,
            ),
            0.3,
        )(area, buffer);
    }
}
```

