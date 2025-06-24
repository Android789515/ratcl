# ratcl

Create complex [ratatui](https://ratatui.rs/) layouts with a simple API.

![showcase](./example-pictures/showcase.png)

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

### Output
![rows](./example-pictures/rows-example.png)

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

### Output
![columns](./example-pictures/columns-example.png)

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
            columns(
              rows(
                  make_cell(InnerStruct::inside(paragraph.clone())),
                  columns(
                      make_cell(paragraph.clone()),
                      make_cell(paragraph.clone()),
                      0.5,
                  ),
                  0.5,
              ),
              columns(
                  rows(
                      make_cell(paragraph.clone()),
                      make_cell(paragraph.clone()),
                      0.5,
                  ),
                  rows(
                      make_cell(paragraph.clone()),
                      make_cell(paragraph.clone()),
                      0.4,
                  ),
                  0.5,
              ),
              0.5,
            ),
            0.3,
        )(area, buffer);

    }
}

#[derive(Clone)]
struct InnerStruct<Content: Widget + Clone> {
    inside: Content,
}

impl <Content: Widget + Clone> InnerStruct<Content> {
    pub fn inside(content: Content) -> impl Widget + Clone {
        Self {
            inside: content,
        }
    }
}

impl <Content: Widget + Clone> Widget for InnerStruct<Content> {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let inner_block = Block::bordered();
        let inner_paragraph = Paragraph::new("Inner")
            .block(inner_block);

        make_cell(self.inside)(area, buffer);

        columns(
            make_cell(inner_paragraph.clone()),
            make_cell(inner_paragraph),
            0.7,
        )(area.inner(Margin::new(4, 2)), buffer);
    }
}
```

### Output
![complex-layout](./example-pictures/complex-layout-example.png)
