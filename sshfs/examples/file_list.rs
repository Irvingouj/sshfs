use crossterm::{
    event::{self, Event},
    terminal,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};
use sshfs::debug::debug_draw;
use std::io;

fn main() -> anyhow::Result<()> {
    debug_draw(|mut terminal| {
        // Create a list of items
        let items = vec![
            ListItem::new(Line::from(Span::raw("Item 1"))),
            ListItem::new(Line::from(Span::raw("Item 2"))),
            ListItem::new(Line::from(Span::raw("Item 3"))),
        ];

        // Create a List widget with the items
        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("My List"))
            .highlight_style(Style::default().fg(Color::Yellow));

        // Render loop
        terminal.draw(|f| {
            let size = f.area();
            let chunks = Layout::default()
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);

            f.render_widget(list, chunks[0]);
        })?;

        Ok(())
    })
}
