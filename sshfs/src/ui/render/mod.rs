use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget},
};

use super::app::App;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Create a "Hello World" span with styling
        let text = "Hello World";
        let span = Span::styled(text, Style::default().fg(Color::Yellow));

        // Calculate the position to center the text in the middle of the area
        let x = area.x + (area.width.saturating_sub(text.len() as u16)) / 2;
        let y = area.y + area.height / 2;

        // Render the span at the calculated position
        buf.set_span(x, y, &span, text.len() as u16);
    }
}
