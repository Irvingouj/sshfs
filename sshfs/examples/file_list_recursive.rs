use anyhow::Context;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::Style,
    text::{Span, Text},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};

enum RecursiveItem<'a> {
    File {
        text: &'a str,
    },
    Directory {
        text: &'a str,
        children: Option<Vec<RecursiveItem<'a>>>,
    },
}

fn build_recursive_list<'a>(items: &'a [RecursiveItem], indent_level: usize) -> Vec<ListItem<'a>> {
    let mut list_items = Vec::new();

    for item in items {
        let indent = " ".repeat(indent_level * 2);
        match item {
            RecursiveItem::File { text } => {
                let text = Text::from(Span::styled(
                    format!("{}{}", indent, text),
                    Style::default(),
                ));
                list_items.push(ListItem::new(text));
            }
            RecursiveItem::Directory { text, children } => {
                let text = Text::from(Span::styled(
                    format!("{}{}", indent, text),
                    Style::default(),
                ));
                list_items.push(ListItem::new(text));

                if let Some(children) = children {
                    list_items.extend(build_recursive_list(children, indent_level + 1));
                }
            }
        }
    }

    list_items
}

fn render_recursive_list(f: &mut ratatui::Frame, area: Rect, items: &[RecursiveItem]) {
    let list_items = build_recursive_list(items, 0);
    let list = List::new(list_items).block(
        Block::default()
            .title("Recursive List")
            .borders(Borders::ALL),
    );
    f.render_widget(list, area);
}

fn main() -> anyhow::Result<()> {
    let items = vec![
        RecursiveItem::Directory {
            text: "Item 1",
            children: Some(vec![
                RecursiveItem::File {
                    text: "Item 1.1",
                },
                RecursiveItem::Directory {
                    text: "Item 1.2",
                    children: Some(vec![
                        RecursiveItem::File {
                            text: "Item 1.2.1",
                        },
                        RecursiveItem::File {
                            text: "Item 1.2.2",
                        },
                    ]),
                },
            ]),
        },
        RecursiveItem::File {
            text: "Item 2",
        },
    ];

    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    terminal.draw(|f| {
        let size = f.area();
        render_recursive_list(f, size, &items);
    }).context("drawing")?;

    Ok(())
}
