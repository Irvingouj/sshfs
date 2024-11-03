use std::path::Path;

use tokio::sync::Mutex;

use super::fs_manager::{self, FsManager};

#[derive(Debug)]
pub struct FsTree {
    current: Mutex<Node>,
}

impl FsTree {
    pub async fn new(cwd: impl AsRef<Path>, fs_manager: &impl FsManager) -> anyhow::Result<Self> {
        let metadata = fs_manager.get_file_metadata(&cwd).await?;

        let children = fs_manager
            .get_children(&cwd)
            .await?
            .into_iter()
            .map(Into::into)
            .collect();

        let parent = fs_manager.get_parent(cwd).await?;

        let current_node = Node {
            metadata,
            children: Some(children),
            parent: Some(Box::new(parent.into())),
            ..Default::default()
        };

        Ok(FsTree {
            current: Mutex::new(current_node),
        })
    }
}

#[derive(Debug, Default)]
pub struct Node {
    pub metadata: crate::state::fs_manager::Metadata,
    pub selected: bool,
    pub expanded: bool,
    pub children: Option<Vec<Node>>,
    pub parent: Option<Box<Node>>,
}

impl From<crate::state::fs_manager::Metadata> for Node {
    fn from(metadata: crate::state::fs_manager::Metadata) -> Self {
        Self {
            metadata,
            ..Default::default()
        }
    }
}
