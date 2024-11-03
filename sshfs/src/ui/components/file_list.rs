use ratatui::widgets::Widget;

use crate::state::fs_tree::FsTree;

pub struct FileList<'a> {
    fs_tree: &'a FsTree,
}

impl<'a> Widget for &FileList<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    {
        todo!()
    }
}
