# ratcl

Create complex layouts with a simple API.

## Create Rows

```rs
use ratatui::{buffer::Buffer, layout::Rect, widgets::{Paragraph, Widget, Block}};
use ratcl::{rows, make_cell};

struct SomeStruct;

impl Widget for SomeStruct {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let block = Block::bordered();
        let paragraph = Paragraph::new("Hello")
            .block(block);

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
use ratatui::{buffer::Buffer, layout::Rect, widgets::{Paragraph, Widget, Block}};
use ratcl::{columns, make_cell};

struct SomeStruct;

impl Widget for SomeStruct {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let block = Block::bordered();
        let paragraph = Paragraph::new("Hello")
            .block(block);

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
use ratatui::{buffer::Buffer, layout::Rect, widgets::{Paragraph, Widget, Block}};
use ratcl::{rows, columns, make_cell};

struct SomeStruct;

impl Widget for SomeStruct {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let block = Block::bordered();
        let paragraph = Paragraph::new("Hello")
            .block(block);

        rows(
            make_cell(paragraph.clone()),
            rows(
              make_cell(paragraph.clone()),
              columns(
                  make_cell(paragraph.clone()),
                  make_cell(paragraph.clone()),
                  0.5,
              ),
              0.5,
            ),
            0.3,
        )(area, buffer);
    }
}
```

