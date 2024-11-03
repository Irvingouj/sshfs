use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::{ListItem, Widget},
};

use crate::state::fs_tree::Node;

pub struct FileListItem<'a> {
    node: &'a Node,
}

impl<'a> FileListItem<'a> {
    pub fn new(node: &'a Node) -> Self {
        Self { node }
    }
}

impl<'a> Widget for &FileListItem<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = &self.node.metadata.name;
        let item = ListItem::new(Line::from(Span::raw(text)));
        // item.render(area, buf);
    }
}
