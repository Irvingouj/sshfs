use crate::state::fs_manager::FsManager;

#[derive(Debug)]
pub struct TokioFsManager {}

impl TokioFsManager {
    pub fn new() -> Self {
        Self {}
    }
}

impl FsManager for TokioFsManager {
    async fn get_file_metadata(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<crate::state::fs_manager::Metadata, anyhow::Error> {
        let file = tokio::fs::File::open(path.as_ref()).await?;
        let metadata = file.metadata().await?;
        let private_metadata = crate::state::fs_manager::Metadata {
            size: metadata.len(),
            created_at: metadata
                .created()?
                .duration_since(std::time::SystemTime::UNIX_EPOCH)?
                .as_secs(),
            name: path
                .as_ref()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string(),
            is_dir: metadata.is_dir(),
            path: path.as_ref().to_path_buf(),
        };

        Ok(private_metadata)
    }

    async fn get_children(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<Vec<crate::state::fs_manager::Metadata>, anyhow::Error> {
        let mut entries = tokio::fs::read_dir(path.as_ref()).await?;
        let mut children = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            let metadata = entry.metadata().await?;
            let child_metadata = crate::state::fs_manager::Metadata {
                size: metadata.len(),
                created_at: metadata
                    .created()?
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)?
                    .as_secs(),
                name: entry.file_name().to_string_lossy().to_string(),
                is_dir: metadata.is_dir(),
                path: entry.path(),
            };
            children.push(child_metadata);
        }

        Ok(children)
    }

    async fn get_parent(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<crate::state::fs_manager::Metadata, anyhow::Error> {
        let parent_path = match path.as_ref().parent() {
            Some(parent) => parent,
            None => return Err(anyhow::anyhow!("No parent directory found")),
        };

        let metadata = tokio::fs::metadata(parent_path).await?;
        let parent_metadata = crate::state::fs_manager::Metadata {
            size: metadata.len(),
            created_at: metadata
                .created()?
                .duration_since(std::time::SystemTime::UNIX_EPOCH)?
                .as_secs(),
            name: parent_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            is_dir: metadata.is_dir(),
            path: parent_path.to_path_buf(),
        };

        Ok(parent_metadata)
    }
}
